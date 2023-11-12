use actix_web::{HttpServer, web, HttpResponse, http::header::ContentType, Responder, App};
use arcs_logging_rs::set_up_logging;



use improved_eureka::env::port_u16_panic;
use improved_eureka::{state::AppState, graphql::Schema};
use improved_eureka::graphql::{schema, save_schema};

use improved_eureka::database::{connect_as, unwrap_connection};
use improved_eureka::logging::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Server process started");

    let (schema, bind_to) = get_setup().await;

    let server = HttpServer::new(move || {
        debug!("Server thread spun up");
        App::new()
            .app_data(schema.clone())
            .service(graphql_handler)
            .service(interactive)
    })
        .bind(bind_to)?
        .run();

    let (result, _) = tokio::join!(
        server,
        async { info!("Server bound to {}:{}", bind_to.0, bind_to.1); },
    );

    result
}

/// This function does most of the "dirty work" of setting up the server.
/// 
/// It's here to keep the main function clean, and it also represents a
/// separation of concerns in that it will reduce the data needed to run the
/// server down to just 2 values.
async fn get_setup() -> (actix_web::web::Data<Schema>, (&'static str, u16)) {
    dotenvy::dotenv().unwrap();
    set_up_logging(&arcs_logging_rs::DEFAULT_LOGGGING_TARGETS, "TableJet Improved Eureka").unwrap();

    {
        use improved_eureka::env::checks::*;
        main().unwrap();
        sql().unwrap();
        sheets().unwrap();
    }


    let db = connect_as("TableJet Improved Eureka").await;
    let db = unwrap_connection(db);
    let ctx: AppState = AppState::new(db);
    info!("Connected to db");

    let schema = schema(ctx);
    info!("Created schema");

    save_schema(&schema, "./schema.graphql");


    let ip = "0.0.0.0";
    let port = port_u16_panic();

    println!("issuing notification...");
    improved_eureka::notifications::notify().await;
    println!("notification sent out;");

    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(actix_web::web::Data::new(schema.clone()))
    //         // .service(
    //         //     web::resource("/")
    //         //         .guard(guard::Post())
    //         //         .to(GraphQL::new(schema)),
    //         // )
    //         // .handler()
    //         // .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    //         .service(graphql_handler)
    //         .service(interactive)
    // })
    //     .bind((ip, port))?
    //     .run()
    //     .await
    (actix_web::web::Data::new(schema), (ip, port))
}



use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

/// This route handles all of the GraphQL requests. It's essentially the basis
/// of the API.
/// 
/// This function is mostly here to bridge an actix endpoint and
/// `async_graphql`'s [`Schema`][async_graphql::Schema], so look in
/// `crate::graphql` for more information.
#[actix_web::post("/graphql", name = "graphql_handler")]
async fn graphql_handler(
    request: GraphQLRequest,
    schema: web::Data<Schema>,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}



/// This endpoint (`/`) handles serving the
/// [`graphiql`][https://www.gatsbyjs.com/docs/how-to/querying-data/running-queries-with-graphiql/]
/// interface for testing queries with Tablejet's API.
/// 
/// This shouldn't be used in production, but it's honestly invaluable in
/// development.
#[actix_web::get("/", name = "Interactive GraphQl Endpoint")]
async fn interactive() -> impl Responder {
    let html_response = async_graphql::http::graphiql_source("/graphql", None);
    let html_response = html_response.replace("Simple GraphiQL Example", "Tablejet Interactive GraphQL API");

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_response)
}


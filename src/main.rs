use actix_web::{HttpServer, web, HttpResponse, http::header::ContentType, Responder, App};
use arcs_logging_rs::set_up_logging;



use improved_eureka::{state::AppState, graphql::Schema};
use improved_eureka::graphql::schema;

use improved_eureka::database::connect_as;
use improved_eureka::logging::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();
    set_up_logging(&arcs_logging_rs::DEFAULT_LOGGGING_TARGETS, "TableJet Improved Eureka").unwrap();

    {
        use improved_eureka::env::checks::*;
        main().unwrap();
        sql().unwrap();
        sheets().unwrap();
    }


    let postgres_connect_result = connect_as("TableJet Improved Eureka").await;
    
    let db = match postgres_connect_result {
        Ok(client) => client,
        Err(e) => {
            error!("failed to connect to eureka db: {e}");
            debug!("Eureka db error: {e:#?}");
            panic!("Failed to connect to eureka db: {e}");
        }
    };
    info!("Connected to db");
    let ctx: AppState = AppState::new(db);

    let schema = schema(ctx); // (true, Some(std::path::Path::new("./schema.graphql"))).unwrap();
    info!("Created schema");

    if let Err(err) = std::fs::write("./schema.graphql", schema.sdl()) {
        warn!("Schema failed to save: {err}");
    } else {
        info!("Schema saved");
    }


    let ip = "0.0.0.0";

    let port = improved_eureka::env::port();

    let Ok(port) = port.parse() else {
        error!("Failed to parse port as u16");
        debug!("Port: {:#?}", port);
        panic!("Failed to parse port as u16");
    };

    let server = HttpServer::new(move || {
        debug!("Server thread spun up");
        App::new()
            .app_data(actix_web::web::Data::new(schema.clone()))
            .service(graphql_handler)
            .service(interactive)
    })
        .bind((ip, port))?
        .run();

    let (result, _) = tokio::join!(
        server,
        async { info!("Server started bound to {ip}:{port}"); },
    );

    result
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


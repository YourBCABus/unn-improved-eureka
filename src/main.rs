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
        info!("Schema failed to save: {err}");
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

    println!("issuing notification...");
    improved_eureka::notifications::notify().await;
    println!("notification sent out;");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(schema.clone()))
            // .service(
            //     web::resource("/")
            //         .guard(guard::Post())
            //         .to(GraphQL::new(schema)),
            // )
            // .handler()
            // .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
            .service(graphql_handler)
            .service(interactive)
    })
        .bind((ip, port))?
        .run()
        .await
}

use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

#[actix_web::post("/", name = "graphql_handler")]
async fn graphql_handler(
    request: GraphQLRequest,
    schema: web::Data<Schema>,
) -> GraphQLResponse {

    schema.execute(request.into_inner()).await.into()
    // json
    //     .into_inner()
    //     .handle()
    //     .await
    //     .unwrap()
    //     .response()

    // if authorization.0.check_matches(&[ Token::Frontend, Token::Deploy ]) {
    // } else {
    //     // TODO: More accurate error messages
    //     HttpResponse::Unauthorized()
    //         .json(json!({ "error": "Improper bearer authentication" }))
    // }
    // HttpResponse::Ok().body(output)
}

#[actix_web::get("/")]
async fn interactive() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(async_graphql::http::graphiql_source("/", None))
    
}


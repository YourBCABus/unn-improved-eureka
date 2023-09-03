use std::sync::Arc;
use actix_web::{HttpServer, web};
use juniper::http::GraphQLRequest;
use warp::Filter;



use improved_eureka::{preludes::{
    graphql::{
        easy_schema,
        exec_graphql,
        graphiql_source,
    },
    utils::structs::*,
    verification::auth_all_method_gen,
}, state::{AppState, WebContext}, graphql};

use improved_eureka::database::connect_as;
use improved_eureka::logging::*;


use warp::hyper::body::Bytes;
use warp::reject;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();
    set_up_logging(&arcs_logging_rs::DEFAULT_LOGGGING_TARGETS, "TableJet Improved Eureka").unwrap();

    {
        use arcs_env_rs::checks::*;
        verify_env!(main: "main");
        verify_env!(sql: "sql");
        verify_env!(discord: "discord");
        verify_env!(auth: "auth");
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
    let schema = easy_schema(true, Some(std::path::Path::new("./schema.graphql"))).unwrap();

    let ctx: AppState = Arc::new(
        WebContext {
            db,
            schema,
        },
    );

    let ip = "0.0.0.0";
    let Ok(port) = env::port().parse() else {
        error!("Failed to parse port as u16");
        debug!("Port: {:#?}", env::port());
        panic!("Failed to parse port as u16");
    };

    HttpServer::new(|| {
        App::new()
            .app_data(actix_web::web::Data::new(ctx))
            .service(main_route)
    })
        .bind((ip, port))?
        .run()
        .await
    
    // let authenticate = auth_all_method_gen();

    // let graphql_route = warp::post()
    //     .and(warp::path!("graphql"))
    //     .and(schema)
    //     .and(ctx)
    //     .and(authenticate)
    //     .map(|sch, ctx, (auth, body): (_, Bytes)| Ok((
    //         sch,
    //         ctx,
    //         auth,
    //         serde_json::from_slice(&body[..])?
    //     )))
    //     .and_then(|result_in: Result<_, BoxError>| async {
    //         result_in.map_err(
    //             |error| reject::custom(BodyDeserializeError::from_cause(error))
    //         )
    //     })
    //     .map(|tup: (_, _, _, _)| {
    //         println!("{:?}", tup.2);
    //         tup
    //     })
    //     .untuple_one()
    //     .and_then(exec_graphql);

    // let graphiql_route = warp::get()
    //     .and(warp::path!("graphiql"))
    //     .map(|| warp::reply::html(graphiql_source("graphql", None)));

    // warp::serve(graphql_route.or(graphiql_route)).run(([127, 0, 0, 1], port)).await;
}

#[actix_web::post("/graphql", name = "graphql_handler")]
async fn main_route(body: Bytes, authorization: Header<AuthHeader>, ctx: web::Data<AppState>) -> impl Responder {
    let req_data: GraphQLRequest = match serde_json::from_slice(&body) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to deserialize body: {e:#?}");
            return HttpResponse::BadRequest()
                .json(json!({ "error": "Failed to deserialize body" }))
        }
    };

    graphql::exec_graphql(&ctx.schema, &ctx.db, req_data);


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
}


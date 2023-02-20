use std::sync::Arc;
use warp::Filter;



use improved_eureka::preludes::{
    graphql::{
        easy_schema,
        exec_graphql,
        graphiql_source,
    },
    database::connect_with,
    utils::structs::*,
    verification::auth_all_method_gen,
};

use warp::hyper::body::Bytes;
use warp::reject;


#[tokio::main]
async fn main() {
    let user = std::env::var("DB_USER").unwrap_or_else(|_| "eureka".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let port = port.parse().unwrap_or(8000);

    let postgres_connect_result = connect_with("localhost", &user).await;
    let db_ctx = match postgres_connect_result {
        Ok(client) => client,
        Err(e) => panic!("failed to connect to eureka db: {}", e),
    };

    let schema = easy_schema(true, Some(std::path::Path::new("./schema.graphql"))).unwrap();


    // Create warp "Filter"s (used as auto-cloned Arcs in this case).
    let schema = warp::any().map(move || Arc::clone(&schema));
    let ctx = warp::any().map(move || Arc::clone(&db_ctx));

    let authenticate = auth_all_method_gen();

    let graphql_route = warp::post()
        .and(warp::path!("graphql"))
        .and(schema)
        .and(ctx)
        .and(authenticate)
        .map(|sch, ctx, (auth, body): (_, Bytes)| Ok((
            sch,
            ctx,
            auth,
            serde_json::from_slice(&body[..])?
        )))
        .and_then(|result_in: Result<_, BoxError>| async {
            result_in.map_err(
                |error| reject::custom(BodyDeserializeError::from_cause(error))
            )
        })
        .map(|tup: (_, _, _, _)| {
            println!("{:?}", tup.2);
            tup
        })
        .untuple_one()
        .and_then(exec_graphql);

    let graphiql_route = warp::get()
        .and(warp::path!("graphiql"))
        .map(|| warp::reply::html(graphiql_source("graphql", None)));

    warp::serve(graphql_route.or(graphiql_route)).run(([127, 0, 0, 1], port)).await;
}

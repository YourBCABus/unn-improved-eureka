use actix_web::web::Header;
use actix_web::{HttpServer, web, HttpResponse, http::header::ContentType, Responder};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use improved_eureka::verification::{ClientSecretHeader, ClientIdHeader};
use improved_eureka::graphql::Schema;

use improved_eureka::logging::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Server process started");

    let sender = setup::metrics();

    setup::env_and_logging();
    let schema = setup::data(
        Some("./schema.graphql"),
        sender.clone(),
    ).await;
    let bind_to = setup::get_bind().await;


    let server = HttpServer::new(
        move || setup::app(schema.clone(), None, sender.clone())
    ).bind(bind_to)?.run();

    let (result, _) = tokio::join!(
        server,
        async { info!("Server bound to {}:{}", bind_to.0, bind_to.1); },
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

    client_id: Option<Header<ClientIdHeader>>,
    client_secret: Option<Header<ClientSecretHeader>>,
) -> GraphQLResponse {    
    let request = augment_request(request.into_inner(), client_id, client_secret).await;
    schema.execute(request).await.into()
}


pub async fn augment_request(
    request: async_graphql::Request,
    client_id: Option<Header<ClientIdHeader>>,
    client_secret: Option<Header<ClientSecretHeader>>,
) -> async_graphql::Request {
    use tokio::sync::OnceCell;
    use improved_eureka::verification::scopes::Scopes;
    let scopes_once_cell: OnceCell<Scopes> = OnceCell::new();
    let request = request.data(scopes_once_cell);

    if let (Some(id), Some(secret)) = (client_id, client_secret) {
        request.data(id.0).data(secret.0)
    } else {
        request
    }
}


/// This endpoint (`/`) handles serving the
/// [`graphiql`][https://www.gatsbyjs.com/docs/how-to/querying-data/running-queries-with-graphiql/]
/// interface for testing queries with Tablejet's API.
/// 
/// This shouldn't be used in production, but it's honestly invaluable in
/// development.
#[actix_web::get("/", name = "Interactive GraphQl Endpoint")]
async fn interactive() -> impl Responder {

    let config = GraphQLPlaygroundConfig::new("/graphql")
        .title("TableJet Interactive GraphQL API");

    let html_response = playground_source(config);
    
    let secured_response = html_response.replace("//cdn.jsdelivr.net", "https://cdn.jsdelivr.net");

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(secured_response)
}


mod setup {
    use improved_eureka::graphql::Schema;

    /// This function sets up the environment using dotenvy and initializes the
    /// logging system.
    /// 
    /// NOTE: We probably should more away from the ARCS thing at some point,
    /// but it works for now.
    pub fn env_and_logging() {
        use arcs_logging_rs::set_up_logging;

        dotenvy::dotenv().unwrap();
        set_up_logging(&arcs_logging_rs::DEFAULT_LOGGGING_TARGETS, "TableJet Improved Eureka").unwrap();

        {
            use improved_eureka::env::checks::*;
            main().unwrap();
            sql().unwrap();
        }
    }

    /// Gets and starts metrics monitoring
    pub fn metrics() -> improved_eureka::metrics::MetricProducer {
        let metrics = improved_eureka::metrics::ResponseTimeMetrics::default();
        let sender = metrics.sender();
        
        metrics.spawn();

        sender
    }

    /// Gets (and unwraps) the db pool connection
    async fn db() -> sqlx::PgPool {
        use improved_eureka::database::{ connect_as, unwrap_connection };

        let db_conn = connect_as("TableJet Improved Eureka").await;
        unwrap_connection(db_conn)
    }

    /// Gets the graphql schema (with the associated db context) for the server
    fn schema(db: sqlx::PgPool, metrics: improved_eureka::metrics::MetricProducer) -> improved_eureka::graphql::Schema {
        use improved_eureka::state::AppState;
        use improved_eureka::graphql::schema;

        let ctx: AppState = AppState::new(db, metrics);
        schema(ctx)
    }


    /// This function gets a `Data<Schema>` struct, ready to be passed to
    /// the application builder.
    pub async fn data(
        save_schema: Option<&str>,
        metrics: improved_eureka::metrics::MetricProducer,
    ) -> actix_web::web::Data<Schema> {
        let db = db().await;
        let schema = schema(db, metrics);
        if let Some(path) = save_schema {
            improved_eureka::graphql::save_schema(&schema, path);
        }
        actix_web::web::Data::new(schema)
    }


    /// This function does most of the "dirty work" of setting up the server.
    /// 
    /// It's here to keep the main function clean, and it also represents a
    /// separation of concerns in that it will reduce the data needed to run the
    /// server down to just 2 values.
    pub async fn get_bind() -> (&'static str, u16) {
        let ip = "0.0.0.0";
        let port = improved_eureka::env::port_u16_panic();
        
        (ip, port)
    }


    pub fn default_cors() -> actix_cors::Cors {
        actix_cors::Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin("https://tbj.yourbcabus.com")
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
    }


    use actix_web::{ App, Error };
    use actix_web::dev::{ ServiceFactory, ServiceRequest, ServiceResponse };
    use actix_web::body::MessageBody;
    use improved_eureka::metrics::{ MetricProducer, middleware::ResponseTimeRecorder };

    /// This function creates an instance of an actix App
    pub fn app(
        schema: actix_web::web::Data<Schema>,
        cors: Option<actix_cors::Cors>,
        metrics: MetricProducer,
    ) -> App<impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        Error = Error,
        InitError = (),
    >> {
        actix_web::App::new()
            .wrap(cors.unwrap_or_else(default_cors))
            .wrap(ResponseTimeRecorder::new(metrics))
            .app_data(schema)
            .service(super::graphql_handler)
            .service(super::interactive)
    }
}


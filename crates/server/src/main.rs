use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello, world!".to_string()
    }
}

type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn index(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

async fn establish_db_connection() -> Result<DatabaseConnection, DbErr> {
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection = Database::connect(&db_url).await?;
    Ok(db)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    match dotenv() {
        Ok(_) => tracing::info!("Loaded .env file"),
        Err(e) => tracing::error!("Failed to load .env file: {}", e),
    }
    match establish_db_connection().await {
        Ok(db) => tracing::info!("Connected to database: {:?}", db),
        Err(e) => tracing::error!("Failed to connect to database: {:?}", e),
    }
    let server_url: String = env::var("SERVER_URL").unwrap();
    let server_port: String = env::var("SERVER_PORT").unwrap();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    tracing::info!("GraphiQL IDE: {:?}:{:?}", server_url, server_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind(format!("{}:{}", server_url, server_port))?
    .run()
    .await
}

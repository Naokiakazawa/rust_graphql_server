use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use std::env;

use graphql::mutations::MutationRoot;
use graphql::queries::QueryRoot;
use graphql::schema::AppSchema;
use infrastructure::database::establish_db_connection;

async fn index(
    schema: web::Data<AppSchema>,
    req: GraphQLRequest,
    db: web::Data<DatabaseConnection>,
) -> GraphQLResponse {
    schema
        .execute(req.into_inner().data(db.clone()))
        .await
        .into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish()))
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_test_writer()
        .init();
    match dotenv() {
        Ok(_) => tracing::info!("Loaded .env file"),
        Err(e) => tracing::error!("Failed to load .env file: {}", e),
    }
    let db = match establish_db_connection().await {
        Ok(db) => {
            tracing::info!("Connected to database");
            db
        }
        Err(e) => {
            tracing::error!("Failed to connect to database: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ));
        }
    };
    let server_url: String = env::var("SERVER_URL").unwrap();
    let server_port: String = env::var("SERVER_PORT").unwrap();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db.clone())
        .finish();

    tracing::info!("GraphiQL IDE: http://localhost:{}", server_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(db.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(web::resource("/graphql").guard(guard::Get()).to(index_graphiql))
    })
    .bind(format!("{}:{}", server_url, server_port))?
    .run()
    .await
}

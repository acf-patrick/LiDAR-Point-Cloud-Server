mod graphql;
mod handlers;
mod database;

use actix_web::{web, App, HttpServer};
use database::Database;
use dotenvy::dotenv;
use graphql::schema::{create_schema, Schema};
use std::sync::{Arc, Mutex};

use handlers::*;

pub struct AppState {
    pub root_node: Schema,
    pub db: Arc<Mutex<Database>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv();
    let port = std::env::var("PORT").unwrap_or(String::from("8080"));

    let app_state = web::Data::new(AppState {
        root_node: create_schema(),
        db: Arc::new(Mutex::new(Database::new())),
    });

    println!("Server running on port {port}");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(root)
            .service(graphiql)
            .service(graphql_handler)
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}

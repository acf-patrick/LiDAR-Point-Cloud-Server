mod graphql;
mod handlers;
mod models;
mod schema;
mod services;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use dotenvy::dotenv;
use graphql::schema::{create_schema, Schema};
use std::sync::{Arc, Mutex};

use handlers::*;

pub struct AppState {
    pub root_node: Schema,
    pub db_conn: Arc<Mutex<SqliteConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("PORT").unwrap_or(String::from("8080"));

    let conn = SqliteConnection::establish(&db_url)
        .expect(format!("Error connecting to {}", db_url).as_str());

    let app_state = web::Data::new(AppState {
        root_node: create_schema(),
        db_conn: Arc::new(Mutex::new(conn)),
    });

    println!("Server running on port 8080");
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

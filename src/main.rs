mod graphql;
mod handlers;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use context::Source;
use diesel::prelude::*;
use dotenvy::dotenv;
use graphql::schema::{create_schema, Schema};
use las::Reader;
use std::sync::{Arc, Mutex};

use graphql::*;
use handlers::*;

pub struct AppState {
    pub root_node: Schema,
    pub source: Source,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let _conn = SqliteConnection::establish(&db_url)
        .expect(format!("Error connecting to {}", db_url).as_str());

    let file_path = if cfg!(debug_assertions) {
        "./assets/point-cloud.las".to_owned()
    } else {
        std::env::var("PC_FILE").expect("PC_FILE must be set for test")
    };
    let reader = Reader::from_path(file_path).unwrap();

    let app_state = web::Data::new(AppState {
        root_node: create_schema(),
        source: Source::Las(Arc::new(Mutex::new(reader))),
    });

    println!("Server running on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(root)
            .service(graphiql)
            .service(graphql_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

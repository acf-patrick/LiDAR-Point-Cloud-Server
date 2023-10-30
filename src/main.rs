mod database;
mod graphql;
mod handlers;
mod services;

use actix_web::{web, App, HttpServer};
use database::Database;
use dotenvy::dotenv;
use graphql::{
    context::{Context, Extractor},
    schema::{create_schema, Schema},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::services::extractors;
use handlers::*;

pub struct AppState {
    pub root_node: Schema,
    pub context: Context,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv();
    let port = std::env::var("PORT").unwrap_or(String::from("8080"));

    let app_state = web::Data::new(AppState {
        root_node: create_schema(),
        context: Context {
            info_extractors: Arc::new(Mutex::new(HashMap::from([
                (
                    "las".to_owned(),
                    Box::new(extractors::las::Extractor::new(false)) as Extractor,
                ),
                (
                    "laz".to_owned(),
                    Box::new(extractors::las::Extractor::new(true)) as Extractor,
                ),
            ]))),
            db: Arc::new(Mutex::new(Database::new())),
        },
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

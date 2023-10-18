mod context;
mod models;
mod queries;
mod schema;

use std::sync::{Arc, Mutex};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use context::Source;
use dotenvy::dotenv;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use las::Reader;
use schema::{create_schema, Schema};

struct AppState {
    root_node: Schema,
    source: Source,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/graphiql")]
async fn graphiql() -> impl Responder {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/graphql")]
async fn graphql(
    app_state: web::Data<AppState>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let res = data.execute(&app_state.root_node, &app_state.source).await;
    serde_json::to_string(&res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv();

    let file_path = std::env::var("PC_FILE").expect("PC_FILE must be set for test");
    let reader = Reader::from_path(file_path).unwrap();

    let app_state = web::Data::new(AppState {
        root_node: create_schema(),
        source: Source::Las(Arc::new(Mutex::new(reader))),
    });

    println!("Server running on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(hello)
            .service(graphiql)
            .service(graphql)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

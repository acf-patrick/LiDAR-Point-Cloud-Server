mod context;
mod models;
mod query;
mod schema;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use las::Reader;
use schema::{create_schema, Schema};
use context::Source;

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
async fn graphql(schema: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let source = Source::Null;
    let res = data.execute(&schema, &source).await;
    serde_json::to_string(&res)
}

// struct AppState {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv();

    let file_path = std::env::var("PC_FILE").expect("PC_FILE must be set for test");
    let mut _reader = Reader::from_path(file_path).unwrap();

    let schema = web::Data::new(create_schema());

    println!("Server running on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(schema.clone())
            .service(hello)
            .service(graphiql)
            .service(graphql)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

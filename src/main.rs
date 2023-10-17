mod models;
mod schema;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use schema::{create_schema, Schema};

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
    let user = data.execute(&schema, &()).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

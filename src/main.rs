mod schema;

use std::sync::{Arc, Mutex};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use schema::{create_schema, mockup_storage, Context, Schema, Storage};

struct AppData {
    schema: Schema,
    storage: Arc<Mutex<Storage>>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/graphql")]
async fn graphql(app_data: web::Data<AppData>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let ctx = Context::new(&app_data.storage);

    let res = data.execute(&app_data.schema, &ctx).await;
    serde_json::to_string(&res)
}

#[get("/graphiql")]
async fn graphiql() -> impl Responder {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppData {
        schema: create_schema(),
        storage: Arc::new(Mutex::new(mockup_storage())),
    });

    println!("Server running on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(hello)
            .service(graphql)
            .service(graphiql)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

mod graphql;
mod models;
mod schema;

use graphql::*;
use std::sync::{Arc, Mutex};
use models::*;
use diesel::prelude::*;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use context::Source;
use dotenvy::dotenv;
use graphql::schema::{create_schema, Schema};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use las::Reader;

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
async fn graphql_handler(
    app_state: web::Data<AppState>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let res = data.execute(&app_state.root_node, &app_state.source).await;
    serde_json::to_string(&res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use self::schema::files::dsl::*;

    let _ = dotenv();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut conn = SqliteConnection::establish(&db_url)
        .expect(format!("Error connecting to {}", db_url).as_str());

    let results = files
        .filter(id.eq("id"))
        .limit(5)
        .select(File::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    for result in results {
      println!("{:#?}", result);
    }

    return Ok(());

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
            .service(hello)
            .service(graphiql)
            .service(graphql_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

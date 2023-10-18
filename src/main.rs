mod models;
mod schema;

use actix_web::{cookie::time::Instant, get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use las::{Read, Reader};
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
    let res = data.execute(&schema, &()).await;
    HttpResponse::Ok().json(res)
}

#[derive(Debug, Default)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let file_path = std::env::var("PC_FILE").expect("PC_FILE must be set for test");
    let mut reader = Reader::from_path(file_path).unwrap();
    let mut point_count = 0;
    let mut max = Point::default();

    let start = Instant::now();
    for point in reader.points() {
        if let Ok(point) = point {
            if point.x > max.x {
                max.x = point.x;
            }
            if point.y > max.y {
                max.y = point.y;
            }
            if point.z > max.z {
                max.z = point.z;
            }
            point_count += 1;
        }

        if point_count >= 3_000_000 {
            // break;
        }
    }
    let end = Instant::now();

    let elapsed = end - start;
    println!("{point_count} points processed in {} seconds", elapsed.as_seconds_f64());

    println!("{:#?}", max);

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

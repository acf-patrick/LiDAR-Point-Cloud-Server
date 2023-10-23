use std::sync::Arc;

use crate::{graphql::context::Context, AppState};
use actix_web::{get, post, web, HttpResponse, Responder};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the point cloud server!")
}

#[get("/graphiql")]
pub async fn graphiql() -> impl Responder {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/graphql")]
pub async fn graphql_handler(
    app_state: web::Data<AppState>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let ctx = Context {
        db_conn: Arc::clone(&app_state.db_conn),
    };
    let res = data.execute(&app_state.root_node, &ctx).await;
    serde_json::to_string(&res)
}

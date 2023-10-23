use crate::graphql::context::Context;
use juniper::graphql_object;

pub struct QueryLas;

#[graphql_object(context = Context, description = "Enables query for Las/Laz files")]
impl QueryLas {
    fn foo() -> &str {
        "Hello"
    }
}

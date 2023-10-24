use crate::graphql::context::Context;
use juniper::graphql_object;

pub struct LasQuery;

#[graphql_object(context = Context, description = "Enables query for Las/Laz files")]
impl LasQuery {
    fn foo() -> &str {
        "Hello"
    }
}

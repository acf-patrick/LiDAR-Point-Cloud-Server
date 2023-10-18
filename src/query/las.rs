use crate::context::Source;
use juniper::graphql_object;

pub struct QueryLas;

#[graphql_object(context = Source)]
impl QueryLas {
    fn testa() -> &str {
        "coucou"
    }
}

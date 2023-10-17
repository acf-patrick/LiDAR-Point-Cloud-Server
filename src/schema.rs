use crate::models::las::Point;
use juniper::*;

/// Abstract type for query root
pub struct Query;

#[graphql_object]
impl Query {
    fn api() -> &str {
        "v1.0.0"
    }

    fn point() -> Point {
        Point::new()
    }
}

/// Abstract type for mutation root
pub struct Mutation;

#[graphql_object]
impl Mutation {
    #[graphql(description = "api version")]
    fn api() -> &str {
        "v1.0.0"
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}

use juniper::*;

use crate::query::las::QueryLas;
use crate::context::Source;

/// Abstract type for query root
pub struct Query;

#[graphql_object(context = Source)]
impl Query {
    fn api() -> &str {
        "v1.0.0"
    }

    fn las() -> QueryLas {
      QueryLas
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

pub type Schema = RootNode<'static, Query, EmptyMutation<Source>, EmptySubscription<Source>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}

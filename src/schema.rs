use juniper::*;

use crate::context::Source;
use crate::queries::las::QueryLas;

/// Abstract type for query root
pub struct Query;

#[graphql_object(context = Source)]
impl Query {
    #[graphql(description = "API version")]
    fn api() -> &str {
        "v1.0.0"
    }

    #[graphql(description = "Node for LAS/LAZ file query")]
    fn las() -> QueryLas {
        QueryLas
    }
}

/// Abstract type for mutation root
pub struct Mutation;

#[graphql_object]
impl Mutation {
    #[graphql(description = "API version")]
    fn api() -> &str {
        "v1.0.0"
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Source>, EmptySubscription<Source>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}

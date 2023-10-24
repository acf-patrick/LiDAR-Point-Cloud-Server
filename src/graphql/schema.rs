use juniper::*;

use super::context::Context;
use super::models::File;
// use super::queries::LasQuery;

/// Abstract type for query root
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    #[graphql(description = "API version")]
    fn api() -> &str {
        "v1.0.0"
    }

    #[graphql(description = "Node for LAS/LAZ file query")]
    fn las(ctx: &Context, id: String) -> Option<File> {
        let mut conn = ctx.db.lock().unwrap();
        Some(File::from(conn.get_file(id)?))
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

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}

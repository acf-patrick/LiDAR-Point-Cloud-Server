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

    #[graphql(name = "part", description = "Get part infos")]
    fn get_file_part_from_db(ctx: &Context, part_id: String) -> Option<File> {
        let mut conn = ctx.db.lock().unwrap();
        Some(File::from(conn.get_part(part_id)?))
    }

    #[graphql(
        name = "parts",
        description = "Get list of parts forming file with given ID"
    )]
    fn get_parts_by_group(ctx: &Context, file_id: String) -> Vec<File> {
        let mut conn = ctx.db.lock().unwrap();
        let files = conn.get_parts(file_id);
        files.iter().map(|file| File::from(file.clone())).collect()
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

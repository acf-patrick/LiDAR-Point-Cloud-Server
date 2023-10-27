mod file;
mod las;

use self::file::*;
use self::las::*;

use super::context::Context;
use super::models::Part;
use juniper::*;

/// Abstract type for query root
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    #[graphql(description = "API version")]
    fn api() -> &str {
        "v1.0.0"
    }

    #[graphql(description = "Entrypoint for Las related queries")]
    fn las() -> LasQuery {
        LasQuery
    }

    #[graphql(description = "Entrypoint for File related queries")]
    fn file(id: String) -> FileQuery {
        FileQuery {
          id
        }
    }

    #[graphql(name = "part", description = "Get part infos")]
    fn get_part_part_from_db(ctx: &Context, part_id: String) -> Option<Part> {
        let mut conn = ctx.db.lock().unwrap();
        Some(Part::from(conn.get_part(part_id)?))
    }

    #[graphql(
        name = "parts",
        description = "Get list of parts forming part with given ID"
    )]
    fn get_parts_by_group(ctx: &Context, part_id: String) -> Vec<Part> {
        let mut conn = ctx.db.lock().unwrap();
        let parts = conn.get_parts(part_id);
        parts.iter().map(|part| Part::from(part.clone())).collect()
    }
}

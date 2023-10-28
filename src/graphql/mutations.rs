use crate::database::FileType;

use super::context::Context;
use juniper::*;

/// Abstract type for mutation root
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    #[graphql(description = "API version")]
    fn api() -> &str {
        "v1.0.0"
    }

    fn split(&self, ctx: &Context, id: String) -> FieldResult<i32> {
        let mut conn = ctx.db.lock()?;

        if let FileType::Monolithic(_) = conn.get_file_type(id) {
            Ok(0)
        } else {
            Err(FieldError::new(
                "Invalid ID provided",
                graphql_value!("Invalid argument : ID doesn't match to an original file"),
            ))
        }
    }
}

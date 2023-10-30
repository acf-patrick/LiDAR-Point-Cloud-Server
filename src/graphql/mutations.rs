use crate::database::FileType;

use super::{context::Context, models::LasInfo};
use juniper::*;

/// Abstract type for mutation root
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    #[graphql(description = "API version")]
    fn api() -> &str {
        "v1.0.0"
    }

    #[graphql(description = "Write file infos into database")]
    fn register(ctx: &Context, id: String) -> FieldResult<LasInfo> {
        let extractors = ctx.info_extractors.lock()?;

        Ok(LasInfo::default())
    }

    #[graphql(description = "Split monolithic file to parts and delete original file")]
    fn split(ctx: &Context, id: String) -> FieldResult<i32> {
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

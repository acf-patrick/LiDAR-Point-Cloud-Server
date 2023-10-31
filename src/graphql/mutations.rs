use super::{context::Context, models::LasInfo};
use crate::database::FileType;
use juniper::*;
use uuid::Uuid;

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
        // check if input is a valid UUID v4
        let _ = Uuid::parse_str(&id)?;

        let ext = ctx.file_format(id.clone())?;

        if let Ok(extractors) = ctx.info_extractors.lock() {
            if let Some(extractor) = extractors.get(&ext) {
                let infos = extractor.extract(id)?;
                Ok(infos)
            } else {
                Err(FieldError::new(
                    format!("Can not read informations from {ext}"),
                    juniper::Value::Null,
                ))
            }
        } else {
            Err(FieldError::new(
                "Failed to perform infos extraction on file",
                graphql_value!("Unable to take lock on extractors"),
            ))
        }
    }

    #[graphql(description = "Split monolithic file to parts and delete original file")]
    fn split(ctx: &Context, id: String) -> FieldResult<i32> {
        // check if input is a valid UUID v4
        let _ = Uuid::parse_str(&id)?;

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

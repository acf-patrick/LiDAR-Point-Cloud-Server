use super::context::Context;
use super::models::{LasInfo, Part};
use juniper::*;
use uuid::Uuid;

/// Abstract type for query root
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    #[graphql(description = "API version")]
    fn api() -> &str {
        "v1.0.0"
    }

    #[graphql(name = "file", description = "Get file information from database")]
    fn get_file_infos(ctx: &Context, id: String) -> FieldResult<LasInfo> {
        // check if input is a valid UUID v4
        let _ = Uuid::parse_str(&id)?;

        let mut conn = ctx.db.lock().map_err(|err| err.to_string())?;

        Err(FieldError::new("", juniper::Value::Null))
    }

    #[graphql(name = "part", description = "Get part infos")]
    fn get_part_part_from_db(ctx: &Context, part_id: String) -> FieldResult<Part> {
        // check if input is a valid UUID v4
        let _ = Uuid::parse_str(&part_id)?;

        match ctx.db.lock() {
            Ok(mut conn) => {
                if let Some(part) = conn.get_part(part_id.clone()) {
                    Ok(Part::from(part))
                } else {
                    Err(FieldError::new(
                        format!("No matching file part found with '{part_id}'"),
                        juniper::Value::Null,
                    ))
                }
            }
            Err(err) => Err(FieldError::new(
                "Failed to establish database connection",
                graphql_value!(err.to_string()),
            )),
        }
    }

    #[graphql(
        name = "parts",
        description = "Get list of parts forming part with given ID"
    )]
    fn get_parts_by_group(ctx: &Context, file_id: String) -> FieldResult<Vec<Part>> {
        // check if input is a valid UUID v4
        let _ = Uuid::parse_str(&file_id)?;

        let mut conn = ctx.db.lock().map_err(|err| {
            FieldError::new(
                "Failed to establish connection to database",
                graphql_value!(err.to_string()),
            )
        })?;
        let parts = conn.get_parts(file_id);
        Ok(parts.iter().map(|part| Part::from(part.clone())).collect())
    }
}

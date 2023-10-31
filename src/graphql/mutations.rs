use super::{context::Context, models::LasInfo};
use crate::database::{models::File as DbFile, FileType};
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
                let infos = extractor.extract(id.clone())?;

                let mut conn = ctx.db.lock().map_err(|err| err.to_string())?;
                conn.register_file(DbFile::from(LasInfoWithId {
                    id,
                    infos: infos.clone(),
                }))?;

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

struct LasInfoWithId {
    infos: LasInfo,
    id: String,
}

impl From<LasInfoWithId> for DbFile {
    fn from(infos: LasInfoWithId) -> Self {
        let id = infos.id;
        let value = infos.infos;

        Self {
            date: value.date,
            file_source_id: value.file_source_id,
            has_color: value.point_format.color as i32,
            has_gps_time: value.point_format.gps_time as i32,
            id,
            is_compressed: value.point_format.compressed as i32,
            max_x: value.max.x as f32,
            max_y: value.max.y as f32,
            max_z: value.max.z as f32,
            min_x: value.min.x as f32,
            min_y: value.min.y as f32,
            min_z: value.min.z as f32,
            number_of_points: value.number_of_points.parse().unwrap(),
            offset_x: value.offset.x as f32,
            offset_y: value.offset.y as f32,
            offset_z: value.offset.z as f32,
            scale_x: value.scale.x as f32,
            scale_y: value.scale.y as f32,
            scale_z: value.scale.z as f32,
            version_major: value.version.major,
            version_minor: value.version.minor,
        }
    }
}

use super::schema::*;
use diesel::{
    prelude::{Identifiable, Insertable, Queryable},
    Selectable,
};

/// Part composing the point cloud file
#[derive(Queryable, Identifiable, Selectable, Insertable, Debug, Clone)]
#[diesel(belongs_to(File))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Part {
    pub id: String,
    pub file_id: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub edge: f32,
}

impl Part {
    pub fn new(file_id: &str, x: f32, y: f32, z: f32, edge: f32) -> Self {
        Part {
            id: uuid::Uuid::new_v4().into(),
            file_id: file_id.to_owned(),
            x,
            y,
            z,
            edge,
        }
    }
}

#[derive(Queryable, Identifiable, Selectable, Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: String,
    pub file_source_id: i32,
    pub version_minor: i32,
    pub version_major: i32,
    pub date: String,
    pub has_gps_time: i32,
    pub has_color: i32,
    pub is_compressed: i32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
    pub number_of_points: i64,
}

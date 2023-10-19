use crate::schema::*;
use diesel::{
    prelude::{Identifiable, Insertable, Queryable},
    Selectable,
};

/// Part composing the point cloud file
#[derive(Queryable, Identifiable, Selectable, Insertable, Debug)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: String,
    pub file_id: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub edge: f32,
}

impl File {
    pub fn new(file_id: &str, x: f32, y: f32, z: f32, edge: f32) -> File {
        File {
            id: uuid::Uuid::new_v4().into(),
            file_id: file_id.to_owned(),
            x,
            y,
            z,
            edge,
        }
    }
}

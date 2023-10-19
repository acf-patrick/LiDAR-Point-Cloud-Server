use crate::schema::*;
use diesel::{
    prelude::{Identifiable, Queryable},
    Selectable,
};

/// Part composing the point cloud file
#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: String,
    pub file_id: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub edge: f32,
}

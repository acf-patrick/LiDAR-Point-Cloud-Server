use diesel::{
    prelude::{Associations, Identifiable, Queryable},
    Selectable,
};

use crate::schema::*;

/// Contains path to the 
#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct File {
    pub id: String,
    pub path: String,
}

/// Part composing the point cloud file
#[derive(Queryable, Identifiable, Selectable, Associations, Debug)]
#[diesel(belongs_to(File))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Part {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub edge: f32,
    pub file_id: String,
}

use diesel::{
    prelude::{Associations, Identifiable, Queryable},
    Selectable,
};

use crate::schema::*;

#[derive(Queryable, Identifiable, Selectable, Debug)]
pub struct File {
    pub id: String,
    pub path: String,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug)]
#[diesel(belongs_to(File))]
pub struct Part {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub edge: f32,
    pub file_id: String,
}

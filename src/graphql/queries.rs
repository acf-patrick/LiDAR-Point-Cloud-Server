use super::models::LasInfo;
use crate::graphql::context::Context;
use juniper::graphql_object;
use las::{Read, Reader};

use std::fs::File;
use std::io::BufReader;

pub struct LasQuery;

#[graphql_object(context = Context, description = "Enables query for Las/Laz files")]
impl LasQuery {
    pub fn infos(id: String) -> Option<LasInfo> {
        let buff = BufReader::new(File::open(format!("files/{id}.laz")).ok()?);
        let reader = Reader::new(buff).ok()?;

        let header = reader.header().clone();
        Some(LasInfo::from(header))
    }

    // pub fn group_infos(file_id: String) -> Option<
}

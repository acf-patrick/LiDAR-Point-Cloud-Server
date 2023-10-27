use crate::graphql::context::Context;
use crate::graphql::models::LasInfo;
use juniper::graphql_object;
use las::{Read, Reader};

use std::fs::File;
use std::io::BufReader;

pub struct LasQuery;

#[graphql_object(context = Context)]
impl LasQuery {
    pub fn infos(id: String) -> Option<LasInfo> {
        let base = std::env::var("PC_FILES_BASE_PATH").expect("PC_FILES_BASE_PATH variable refers to the folder containing sliced or original LAS files");
        let buff = BufReader::new(File::open(format!("{base}/{id}.laz")).ok()?);
        let reader = Reader::new(buff).ok()?;

        let header = reader.header().clone();
        Some(LasInfo::from(header))
    }

    // pub fn group_infos(file_id: String) -> Option<
}

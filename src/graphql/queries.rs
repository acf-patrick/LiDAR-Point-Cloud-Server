use super::models::LasInfo;
use crate::graphql::context::Context;
use juniper::graphql_object;
use las::{Read, Reader};

pub struct LasQuery {
    pub id: String,
}

#[graphql_object(context = Context, description = "Enables query for Las/Laz files")]
impl LasQuery {
    pub fn infos(&self) -> Option<LasInfo> {
        let reader = Reader::from_path(format!("files/{}.laz", self.id)).ok()?;

        let header = reader.header().clone();
        Some(LasInfo::from(header))
    }
}

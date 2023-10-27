use crate::graphql::{context::Context, models::LasInfo};
use juniper::*;

pub struct FileQuery {
    pub id: String,
}

#[graphql_object(context = Context)]
impl FileQuery {
    fn infos(&self, ctx: &Context) -> Option<LasInfo> {
        let mut conn = ctx.db.lock().ok()?;
        conn.get_file(self.id.clone())
            .map(|record| LasInfo::from(record))
    }
}

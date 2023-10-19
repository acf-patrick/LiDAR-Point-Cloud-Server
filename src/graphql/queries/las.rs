use crate::graphql::models::las::Point;
use crate::graphql::context::Source;
use juniper::graphql_object;
use las::Read;

pub struct QueryLas;

#[graphql_object(context = Source, description = "Enables query for Las/Laz files")]
impl QueryLas {
    #[graphql(description = "Query first point")]
    fn first_point(ctx: &Source) -> Option<Point> {
        if let Source::Las(source) = ctx {
            let mut source = source.lock().ok()?;
            Some(Point::from(source.points().next()?.ok()?))
        } else {
            None
        }
    }
}
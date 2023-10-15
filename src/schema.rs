use juniper::*;

#[derive(GraphQLObject)]
#[graphql(description = "A member of a team")]
struct Member {
    id: i32,
    name: String,
}

pub struct QueryRoot;

#[graphql_object]
impl QueryRoot {
    fn members() -> Vec<Member> {
        vec![
            Member {
                id: 1,
                name: "Link".to_owned(),
            },
            Member {
                id: 2,
                name: "Mario".to_owned(),
            },
        ]
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

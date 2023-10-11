use juniper::*;

#[derive(GraphQLObject)]
#[graphql(description = "A Todo")]
struct Todo {
    id: i32,
    title: String,
    description: String,
    completed: bool,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A Todo to insert")]
struct NewTodo {
    title: String,
    description: String,
    completed: bool,
}

struct Query;

#[graphql_object]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    fn todos() -> Vec<Todo> {
        vec![
            Todo {
                id: 1,
                title: "Watching Basketball".to_owned(),
                description: "Watching the NBA finals".to_owned(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "Watching Football".to_owned(),
                description: "Watching the NFL".to_owned(),
                completed: false,
            },
        ]
    }
}

struct Mutation;

#[graphql_object]
impl Mutation {
    fn createTodo(new_todo: NewTodo) -> Todo {
        Todo {
            id: 1,
            title: new_todo.title,
            description: new_todo.description,
            completed: new_todo.completed,
        }
    }
}

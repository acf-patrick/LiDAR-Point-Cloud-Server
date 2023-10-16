use std::sync::{Arc, Mutex};

use juniper::*;

pub struct Storage {
    members: Vec<Member>,
    teams: Vec<Team>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            members: vec![],
            teams: vec![],
        }
    }
}

pub fn mockup_storage() -> Storage {
    let teams = vec![
        Team {
            id: 1,
            name: "Heroes".to_owned(),
        },
        Team {
            id: 2,
            name: "Villains".to_owned(),
        },
    ];

    let members = vec![
        Member {
            id: 1,
            name: "Link".to_owned(),
            knockouts: 14,
            team_id: 1,
        },
        Member {
            id: 2,
            name: "Mario".to_owned(),
            knockouts: 11,
            team_id: 1,
        },
        Member {
            id: 3,
            name: "Kirby".to_owned(),
            knockouts: 8,
            team_id: 1,
        },
        Member {
            id: 4,
            name: "Ganondorf".to_owned(),
            knockouts: 8,
            team_id: 2,
        },
        Member {
            id: 5,
            name: "Bowser".to_owned(),
            knockouts: 11,
            team_id: 2,
        },
        Member {
            id: 6,
            name: "Mewtwo".to_owned(),
            knockouts: 12,
            team_id: 2,
        },
    ];

    Storage { members, teams }
}

pub struct Context {
    storage: Arc<Mutex<Storage>>,
}

#[allow(dead_code)]
impl Context {
    pub fn new(storage: &Arc<Mutex<Storage>>) -> Context {
        Context {
            storage: Arc::clone(storage),
        }
    }

    pub fn members(&self) -> Vec<Member> {
        if let Ok(storage) = self.storage.lock() {
            storage.members.clone()
        } else {
            vec![]
        }
    }

    pub fn update_members<T>(&self, callback: T)
    where
        T: Fn(&mut Vec<Member>),
    {
        if let Ok(mut storage) = self.storage.lock() {
            callback(&mut storage.members);
        }
    }

    pub fn teams(&self) -> Vec<Team> {
        if let Ok(storage) = self.storage.lock() {
            storage.teams.clone()
        } else {
            vec![]
        }
    }

    pub fn update_teams<T>(&self, callback: T)
    where
        T: Fn(&mut Vec<Team>),
    {
        if let Ok(mut storage) = self.storage.lock() {
            callback(&mut storage.teams);
        }
    }
}

impl juniper::Context for Context {}

#[derive(Clone)]
pub struct Member {
    id: i32,
    name: String,
    knockouts: i32,
    team_id: i32,
}

#[graphql_object(description = "A member of a team")]
impl Member {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn knockouts(&self) -> i32 {
        self.knockouts
    }

    fn team_id(&self) -> i32 {
        self.team_id
    }
}

#[derive(Clone)]
pub struct Team {
    id: i32,
    name: String,
}

#[graphql_object(
  description = "A team of members",
  context = Context,
)]
impl Team {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn members(&self, ctx: &Context) -> Vec<Member> {
        let id = self.id;
        ctx.members()
            .iter()
            .filter_map(|member| {
                if member.team_id == id {
                    Some(member.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    fn members(ctx: &Context) -> Vec<Member> {
        ctx.members()
    }

    fn teams(ctx: &Context) -> Vec<Team> {
        ctx.teams()
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

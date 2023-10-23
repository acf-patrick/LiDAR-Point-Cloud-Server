use diesel::SqliteConnection;
use std::sync::{Arc, Mutex};

pub struct Context {
    pub db_conn: Arc<Mutex<SqliteConnection>>,
}

impl juniper::Context for Context {}

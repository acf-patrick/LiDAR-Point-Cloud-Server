use std::sync::{Arc, Mutex};

use crate::database::Database;

pub struct Context {
    pub db: Arc<Mutex<Database>>,
}

impl juniper::Context for Context {}

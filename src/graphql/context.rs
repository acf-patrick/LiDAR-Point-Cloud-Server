use std::{sync::{Arc, Mutex}, collections::HashMap};

use crate::{database::Database, services::Extract};

pub type Extractor = Box<dyn Extract + Send + Sync>;

pub struct Context {
    pub db: Arc<Mutex<Database>>,
    pub info_extractors: Arc<Mutex<HashMap<String, Extractor>>>
}

impl juniper::Context for Context {}

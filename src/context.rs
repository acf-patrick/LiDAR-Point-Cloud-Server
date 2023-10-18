use las::Reader;
use std::sync::{Arc, Mutex};

pub enum Source {
    Las(Arc<Mutex<Reader<'static>>>),
    Null,
}

impl juniper::Context for Source {}

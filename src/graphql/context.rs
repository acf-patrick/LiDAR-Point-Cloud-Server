use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};

use crate::{database::Database, services::Extract};

pub type Extractor = Box<dyn Extract + Send + Sync>;

pub struct Context {
    pub db: Arc<Mutex<Database>>,
    pub info_extractors: Arc<Mutex<HashMap<String, Extractor>>>,
}

impl juniper::Context for Context {}

impl Context {
    /// Check what kind of file maps to the given ID. Otherwise returns None.
    pub fn file_format(&self, id: String) -> Result<String, String> {
        let base_path = std::env::var("PC_FILES_BASE_PATH").map_err(|err| err.to_string())?;
        let entries = fs::read_dir(base_path).map_err(|err| err.to_string())?;
        let mut ext = Option::<String>::None;

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if let Some(Some(file_name)) = path.file_stem().map(|stem| stem.to_str()) {
                        if file_name.to_owned() == id {
                            ext = Some(extension.to_str().unwrap().to_owned());
                            break;
                        }
                    }
                }
            }
        }

        if let Some(ext) = ext {
            Ok(ext)
        } else {
            Err(format!("No matching file found with given ID '{id}'"))
        }
    }

    /// Check if there is a file mapping to the given ID
    pub fn file_exist(&self, id: String) -> bool {
        self.file_format(id).is_ok()
    }
}

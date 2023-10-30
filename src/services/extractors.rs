/// This module is dedicated for file infos extractors.
/// Create a new module for the desired file extension and implement ```trait InfoExtractor``` for your type

pub mod las {
    use super::super::InfoExtractor;
    use crate::graphql::models::LasInfo;
    use las::{Read, Reader};
    use std::{fs::File, io::BufReader};

    /// Las infos extractor
    pub struct Extractor {
        pub is_compressed: bool,
    }

    impl InfoExtractor for Extractor {
        fn extract(&self, file_id: String) -> Result<LasInfo, String> {
            let base_path = std::env::var("PC_FILES_BASE_PATH")
                .map_err(|_| "PC_FILES_BASE_PATH variable should be set")?;

            let buf = BufReader::new(
                File::open(format!(
                    "{base_path}/{file_id}.{}",
                    if self.is_compressed { "laz" } else { "las" }
                ))
                .map_err(|err| err.to_string())?,
            );

            let reader = Reader::new(buf)
                .map_err(|err| format!("Failed to read point cloud file : {err}"))?;
            let header = reader.header();

            Ok(LasInfo::from(header.clone()))
        }
    }
}

pub mod extractors;

use crate::graphql::models::LasInfo;

/// One has to implement this trait to fetch info from a variaous point cloud file extensions
pub trait InfoExtractor {
    /// Extract header from point cloud file. Refer to ```crate::graphql::models::LasInfo```
    ///
    /// ## Argument
    /// Takes file's ID as argument (aka the filename without extension)
    ///
    /// ## Returns
    /// Informations about the point cloud file or a String descripting the problem
    fn extract(&self, file_id: String) -> Result<LasInfo, String>;
}

/// One has to implement this trait to split various point cloud file extensions
pub trait Splitter {
    /// Split point cloud as voxels
    /// 
    /// ## Argument
    /// Takes file's ID as argument (aka the filename without extension)
    /// 
    /// ## Returns
    /// Number of parts (cube) composing the file or a String descripting the problem
    fn split(&self, file_id: String) -> Result<usize, String>;
}

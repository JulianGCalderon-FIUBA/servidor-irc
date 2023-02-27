use std::path::PathBuf;
/// Saves information about downloaded files.
pub struct Download {
    pub client: String,
    pub name: String,
    pub path: PathBuf,
    pub failed: bool,
}

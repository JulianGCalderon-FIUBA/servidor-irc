use std::path::PathBuf;

pub struct Download {
    pub client: String,
    pub name: String,
    pub path: PathBuf,
    pub failed: bool,
}

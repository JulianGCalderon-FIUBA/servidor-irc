use std::path::PathBuf;

pub struct Transfer {
    pub client: String,
    pub name: String,
    pub path: PathBuf,
    pub failed: bool,
}

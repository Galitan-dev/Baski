use rocket::{response::NamedFile, Route};
use std::path::{Path, PathBuf};

pub fn routes() -> Vec<Route> {
    routes![file]
}

#[get("/<file..>", rank = 0)]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}
use std::path::{Path, PathBuf};

use rocket::{response::NamedFile, Route};

pub fn routes() -> Vec<Route> {
    routes![file]
}

#[get("/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

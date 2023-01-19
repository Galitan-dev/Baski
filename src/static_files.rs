use rocket::{response::NamedFile, Route};
use std::path::{Path, PathBuf};

pub fn get_routes() -> Vec<Route> {
    routes![file]
}

#[get("/<file..>", rank = 0)]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    let public_dir = Path::new("public/");
    let path = public_dir.join(file);

    if path.extension().is_some() {
        NamedFile::open(path).ok()
    } else {
        if path.is_dir() {
            NamedFile::open(path.join("index.html")).ok()
        } else {
            NamedFile::open(path.with_extension("html")).ok()
        }
    }
}
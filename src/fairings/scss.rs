use std::path::PathBuf;

use super::loader::{Info, Loader};

pub struct SCSSLoader;

impl Loader for SCSSLoader {
    fn new() -> Self {
        Self {}
    }

    fn duplicate(&self) -> Self {
        Self {}
    }

    fn info() -> Info {
        Info {
            name: "SCSS",
            loader_name: "SCSS Loader",
            source_directory: PathBuf::from("app/scss"),
            output_directory: PathBuf::from("static/css"),
            output_extension: "css",
        }
    }

    fn compile(&self, path: PathBuf) -> Result<String, String> {
        grass::from_path(path.clone(), &Self::grass_options()).map_err(|e| e.as_ref().to_string())
    }
}

impl SCSSLoader {
    fn grass_options<'a>() -> grass::Options<'a> {
        grass::Options::default()
            .style(grass::OutputStyle::Compressed)
            .input_syntax(grass::InputSyntax::Scss)
    }
}

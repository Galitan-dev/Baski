use std::{path::PathBuf, fs};

use super::{Loader, Info};

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
            source_directory: PathBuf::from("src/scss"),
            output_directory: PathBuf::from("static/css"),
            output_extension: "css",
        }
    }

    fn compile_string(&self, _source: String) -> Result<String, String> {
        panic!("Not allowed")
    }

    fn compile_file(&self, path: PathBuf) {
        match grass::from_path(path.clone(), &Self::grass_options()) {
            Ok(css) => {
                let out_path = Self::info().output_directory.join(path.with_extension(Self::info().output_extension).file_name().unwrap());
                fs::write(out_path, css).unwrap();
                println!("Compiled {}", path.file_name().unwrap().to_str().unwrap())
            }
            Err(err) => {
                println!("\n{}", err.as_ref());
            }
        }
    }
}

impl SCSSLoader {
    fn grass_options<'a>() -> grass::Options<'a> {
        grass::Options::default()
            .style(grass::OutputStyle::Compressed)
            .input_syntax(grass::InputSyntax::Scss)
    }
}

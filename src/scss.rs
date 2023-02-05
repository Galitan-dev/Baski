use std::{
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::channel,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use notify::{DebouncedEvent, RecursiveMode, Watcher};
use rocket::{
    config::Environment,
    fairing::{Fairing, Info, Kind},
};

pub struct SCSSLoader {
    hot_reload: AtomicBool,
    scss_directory: PathBuf,
    css_directory: PathBuf,
}

impl SCSSLoader {
    pub fn new() -> Self {
        Self {
            hot_reload: AtomicBool::new(false),
            scss_directory: PathBuf::from("src/scss"),
            css_directory: PathBuf::from("static/css"),
        }
    }

    fn compile(&self) {
        fs::remove_dir_all(self.css_directory.clone()).unwrap();
        fs::create_dir(self.css_directory.clone()).unwrap();
        for entry in self
            .scss_directory
            .read_dir()
            .expect("Unable to list scss files")
            .flatten()
        {
            if entry.path().is_file() {
                self.compile_file(entry.path())
            }
        }
    }

    fn compile_file(&self, path: PathBuf) {
        match grass::from_path(path.clone(), &self.grass_options()) {
            Ok(css) => {
                let out_path = self
                    .css_directory
                    .join(path.with_extension("css").file_name().unwrap());
                fs::write(out_path, css).unwrap();
                println!("Compiled {}", path.file_name().unwrap().to_str().unwrap())
            }
            Err(err) => {
                println!("\n{}", err.as_ref());
            }
        }
    }

    fn grass_options<'a>(&self) -> grass::Options<'a> {
        grass::Options::default()
            .style(grass::OutputStyle::Compressed)
            .input_syntax(grass::InputSyntax::Scss)
            .load_path(self.css_directory.clone())
    }

    fn enable_hot_reloading(&self) -> JoinHandle<()> {
        let loader = Self {
            css_directory: self.css_directory.clone(),
            scss_directory: self.scss_directory.clone(),
            hot_reload: AtomicBool::new(true),
        };
        thread::spawn(move || {
            println!("⚡️ SCSS Hot Reloading enabled");

            let (tx, rx) = channel::<DebouncedEvent>();

            let mut watcher = notify::watcher(tx, Duration::from_millis(100)).unwrap();

            watcher
                .watch(&loader.scss_directory, RecursiveMode::NonRecursive)
                .unwrap();

            loop {
                match rx.recv() {
                    Ok(event) => match event {
                        DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
                            loader.compile_file(path)
                        }
                        DebouncedEvent::Remove(path) => {
                            let css_path = loader
                                .css_directory
                                .join(path.with_extension("css").file_name().unwrap());
                            if css_path.exists() {
                                fs::remove_file(css_path).unwrap();
                            }
                        }
                        DebouncedEvent::Rename(old, new) => {
                            let css_path = loader
                                .css_directory
                                .join(old.with_extension("css").file_name().unwrap());
                            if css_path.exists() {
                                fs::remove_file(css_path).unwrap();
                            }

                            loader.compile_file(new);
                        }
                        _ => (),
                    },
                    Err(e) => println!("watch error: {e:?}"),
                }
            }
        })
    }
}

impl Fairing for SCSSLoader {
    fn info(&self) -> Info {
        Info {
            name: "SCSS Loader",
            kind: Kind::Launch,
        }
    }

    fn on_launch(&self, rocket: &rocket::Rocket) {
        self.hot_reload.fetch_or(
            rocket.config().environment == Environment::Development,
            Ordering::SeqCst,
        );
        self.compile();
        if self.hot_reload.load(Ordering::SeqCst) {
            self.enable_hot_reloading();
        }
    }
}

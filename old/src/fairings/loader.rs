use std::{
    fs,
    path::PathBuf,
    sync::mpsc::channel,
    thread::{self, JoinHandle},
    time::Duration,
};

use notify::{DebouncedEvent, RecursiveMode, Watcher};
use rocket::fairing::{Fairing, Kind};
pub trait Loader
where
    Self: Sized + Send + 'static,
{
    fn info() -> Info;
    fn new() -> Self;

    fn compile(&self, path: PathBuf) -> Result<String, String>;
    fn duplicate(&self) -> Self;

    fn fairing() -> LoaderFairing<Self> {
        LoaderFairing {
            loader: Self::new(),
        }
    }

    fn as_fairing(self) -> LoaderFairing<Self> {
        LoaderFairing { loader: self }
    }

    fn compile_files(&self) {
        if Self::info().output_directory.exists() {
            fs::remove_dir_all(Self::info().output_directory).unwrap();
        }
        fs::create_dir(Self::info().output_directory).unwrap();
        for entry in Self::info()
            .source_directory
            .read_dir()
            .expect("Unable to list source files")
            .flatten()
        {
            if entry.path().is_file() {
                self.compile_file(entry.path())
            }
        }
    }

    fn compile_file(&self, path: PathBuf) {
        match self.compile(path.clone()) {
            Ok(out) => {
                let out_path = Self::info().output_directory.join(
                    path.with_extension(Self::info().output_extension)
                        .file_name()
                        .unwrap(),
                );
                fs::write(out_path, out).unwrap();
                println!("Compiled {}", path.file_name().unwrap().to_str().unwrap())
            }
            Err(err) => {
                println!("\n{err}");
            }
        }
    }

    fn enable_hot_reloading(&self) -> JoinHandle<()> {
        let loader = self.duplicate();
        let source_directory = Self::info().source_directory;
        let output_directory = Self::info().output_directory;

        thread::spawn(move || {
            println!("⚡️ {} Hot Reloading enabled", Self::info().name);

            let (tx, rx) = channel::<DebouncedEvent>();

            let mut watcher = notify::watcher(tx, Duration::from_millis(100)).unwrap();

            watcher
                .watch(&source_directory, RecursiveMode::NonRecursive)
                .unwrap();

            loop {
                match rx.recv() {
                    Ok(event) => match event {
                        DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
                            loader.compile_file(path)
                        }
                        DebouncedEvent::Remove(path) => {
                            let css_path = output_directory
                                .join(path.with_extension("css").file_name().unwrap());
                            if css_path.exists() {
                                fs::remove_file(css_path).unwrap();
                            }
                        }
                        DebouncedEvent::Rename(old, new) => {
                            let css_path = output_directory
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

pub struct Info {
    pub name: &'static str,
    pub loader_name: &'static str,
    pub source_directory: PathBuf,
    pub output_directory: PathBuf,
    pub output_extension: &'static str,
}

pub struct LoaderFairing<L: Loader> {
    loader: L,
}

impl<L: Loader + Sync + 'static> Fairing for LoaderFairing<L> {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: L::info().loader_name,
            kind: Kind::Launch,
        }
    }

    fn on_launch(&self, rocket: &rocket::Rocket) {
        self.loader.compile_files();
        if rocket.config().get_bool("hot_reload").unwrap_or(false) {
            self.loader.enable_hot_reloading();
        }
    }
}

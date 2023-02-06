use std::process::Command;

use rocket::fairing::{Fairing, Info, Kind};

pub struct TypeScriptLoader;

impl TypeScriptLoader {
    pub fn fairing() -> Self {
        Self {}
    }
}

impl Fairing for TypeScriptLoader {
    fn info(&self) -> Info {
        Info {
            name: "TypeScript Loader",
            kind: Kind::Launch,
        }
    }

    fn on_launch(&self, rocket: &rocket::Rocket) {
        let mut cmd = Command::new("tsc");

        if rocket.config().get_bool("hot_reload").unwrap_or(false) {
            cmd.arg("--watch").arg("--preserveWatchOutput");
        }

        cmd.spawn().expect("Couldn't run tsc");
    }
}

use std::{process::{Command, Child}, io::Error};

use crate::CONFIG;

pub fn load() -> Result<Child, Error> {
    let mut lessc = Command::new("pnpm");
    lessc.arg("less-watch-compiler");

    if !CONFIG.live_reloading {
        lessc.arg("--run-once");
    }

    lessc.spawn()
}
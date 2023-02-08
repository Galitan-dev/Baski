use std::{
    io::{Error, Read},
    process::{Command, Stdio},
    thread,
};

use colored::Colorize;
use regex::Regex;

use crate::{config::LogLevel, CONFIG};

pub fn load() -> Result<(), Error> {
    let mut tsc = Command::new("pnpm");
    tsc.arg("tsc");

    if CONFIG.live_reloading {
        tsc.args(["--watch", "--preserveWatchOutput"]);
    }

    let mut stdout = tsc.stdout(Stdio::piped()).spawn()?.stdout.take().unwrap();

    thread::spawn(move || loop {
        let mut readbuf = [0; 256];
        stdout.read(&mut readbuf).unwrap();
        let string = String::from_utf8_lossy(&readbuf);

        for line in string.lines() {
            if line.starts_with("web/ts") {
                let error = Regex::new(r"(?P<file>[^:]+\(\d+,\d+\)):")
                    .unwrap()
                    .replace_all(line, format!("{}", "${file}:".bright_blue().bold()));

                let error = Regex::new(r"error (?P<number>[^:]+):")
                    .unwrap()
                    .replace_all(
                        &error,
                        format!("{} {}", "error".red(), "${number}:".red().bold()),
                    );

                let error = Regex::new(r"'(?P<content>[^']+)'")
                    .unwrap()
                    .replace_all(&error, format!("{}", "'${content}'".green().italic()));

                println!("{error}");
            }
        }

        if CONFIG.log_level != LogLevel::Error
            && string.contains("Starting compilation in watch mode...")
        {
            println!(
                "{} {} {} {}{}",
                "⚡️ Live Reloading for".yellow(),
                "TypeScript".blue().bold(),
                "files".yellow(),
                "enabled".green(),
                "!".yellow()
            )
        }

        if CONFIG.log_level == LogLevel::All && string.contains("File change detected.") {
            println!(
                "👀 {} {} {} {}{}",
                "a".yellow(),
                "TypeScript".bright_blue().bold(),
                "file has changed 👉 compiling".yellow(),
                "incrementaly".bright_blue().bold(),
                "!".yellow()
            );
        }
    });

    Ok(())
}

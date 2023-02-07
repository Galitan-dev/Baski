use std::{
    io::{Error, Read},
    process::{Command, Stdio},
    thread,
};

use colored::Colorize;
use regex::Regex;

use crate::{config::LogLevel, CONFIG};

pub fn load() -> Result<(), Error> {
    let mut lessc = Command::new("pnpm");
    lessc.arg("less-watch-compiler");

    if !CONFIG.live_reloading {
        lessc.arg("--run-once");
    }

    let mut stdout = lessc.stdout(Stdio::piped()).spawn()?.stdout.take().unwrap();

    thread::spawn(move || loop {
        let mut readbuf = [0; 1024];
        stdout.read(&mut readbuf).unwrap();
        let string = String::from_utf8_lossy(&readbuf);

        if string.contains("Error:") {
            let mut error_lines: Vec<&str> = Vec::new();
            for line in string.lines() {
                if line.trim().starts_with("at ChildProcess.exithandler") {
                    break;
                }
                error_lines.push(line)
            }

            println!("{}", error_lines[1..error_lines.len() - 2].join("\n"));
        }

        if CONFIG.log_level != LogLevel::Error
            && string.contains("Watching directory for file changes.")
        {
            println!(
                "{} {} {} {}{}",
                "⚡️ Live Reloading for".yellow(),
                "LESS".bright_purple().bold(),
                "files".yellow(),
                "enabled".green(),
                "!".yellow()
            )
        }

        if CONFIG.log_level == LogLevel::All && string.contains("Recompiling") {
            let re = Regex::new(r#"(less|css)/([^" ]+)"#).unwrap();
            let mut changed_file = "";
            let mut recompiled_file = "";
            for cap in re.captures_iter(&string) {
                match cap.get(1).unwrap().as_str() {
                    "less" => changed_file = cap.get(2).unwrap().as_str(),
                    "css" => recompiled_file = cap.get(2).unwrap().as_str(),
                    _ => (),
                }
            }
            println!(
                "👀 {} {} {}{}",
                changed_file.bright_purple().bold(),
                "has changed 👉 recompiling".yellow(),
                recompiled_file.bright_purple().bold(),
                "!".yellow()
            );
        }
    });

    Ok(())
}

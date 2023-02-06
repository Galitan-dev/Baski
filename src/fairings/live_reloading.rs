use std::{io::Cursor, sync::mpsc::channel, thread, time::Duration};

use notify::{DebouncedEvent, RecursiveMode, Watcher};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{uri::Origin, ContentType},
    response::Body,
};
pub struct LiveReloading;

static mut ENABLED: bool = false;
static mut RELOAD_FLAG: bool = false;

impl LiveReloading {
    pub fn fairing() -> Self {
        Self {}
    }

    fn enable_live_reloading(&self) {
        println!("⚡️ Live Reloading Enabled");

        thread::spawn(|| {
            let (tx, rx) = channel();

            let mut watcher = notify::watcher(tx, Duration::from_secs(1)).unwrap();

            watcher.watch("app", RecursiveMode::Recursive).unwrap();

            loop {
                match rx.recv() {
                    Ok(event) => match event {
                        DebouncedEvent::Chmod(_)
                        | DebouncedEvent::Error(_, _)
                        | DebouncedEvent::Rescan
                        | DebouncedEvent::NoticeRemove(_)
                        | DebouncedEvent::NoticeWrite(_) => (),
                        _ => unsafe { RELOAD_FLAG = true },
                    },
                    Err(e) => println!("watch error: {e:?}"),
                }
            }
        });
    }
}

impl Fairing for LiveReloading {
    fn info(&self) -> Info {
        Info {
            name: "Live Reloading",
            kind: Kind::Attach | Kind::Launch | Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut rocket::Request, _data: &rocket::Data) {
        if request.uri().path() == "/poll/live_reloading" {
            request.set_uri(
                Origin::parse(if unsafe { RELOAD_FLAG } {
                    "/poll/live_reloading/reload"
                } else {
                    "/poll/live_reloading/idle"
                })
                .unwrap(),
            );
            unsafe { RELOAD_FLAG = false }
        }
    }

    fn on_response(&self, _request: &rocket::Request, response: &mut rocket::Response) {
        if unsafe { ENABLED } {
            if response
                .content_type()
                .map(|c| c == ContentType::HTML)
                .unwrap_or(false)
            {
                if let Some(html) = response.body_string() {
                    let html = html.replace(
                        "</body>",
                        "<script src=\"/static/js/live_reloading.js\"></script>\n</body>",
                    );

                    response.set_raw_body(Body::Sized(Cursor::new(html.clone()), html.len() as _))
                }
            }
        }
    }

    fn on_attach(&self, rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
        if rocket.config().get_bool("hot_reload").unwrap_or(false) {
            unsafe {
                ENABLED = true;
            }
            Ok(rocket.mount("/poll/live_reloading", routes![reload, idle]))
        } else {
            Ok(rocket)
        }
    }

    fn on_launch(&self, _rocket: &rocket::Rocket) {
        if unsafe { ENABLED } {
            self.enable_live_reloading();
        }
    }
}

#[get("/reload")]
fn reload() -> &'static str {
    return "reload";
}

#[get("/idle")]
fn idle() -> &'static str {
    return "idle";
}

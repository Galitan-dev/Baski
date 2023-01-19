#![feature(proc_macro_hygiene, decl_macro)]

use rocket::Rocket;
use rocket_contrib::templates::Template;

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rocket_contrib;

mod api;
mod assets;
mod templates;

fn main() {
    rocket().launch();
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/api", api::routes())
        .mount("/assets", assets::routes())
        .mount("/", templates::routes())
        .attach(Template::fairing())
}
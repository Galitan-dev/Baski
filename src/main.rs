#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::templates::Template;

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rocket_contrib;

mod api;
mod static_files;
mod templates;

fn main() {
    rocket::ignite()
        .mount("/api", api::get_routes())
        .mount("/", templates::get_routes())
        .mount("/", static_files::get_routes())
        .attach(Template::fairing())
        .launch();
}

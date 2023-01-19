#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use(get, routes)]
extern crate rocket;
#[macro_use(Serialize)]
extern crate serde_derive;
extern crate rocket_contrib;
extern crate serde_json;

use rocket::Rocket;
use rocket_contrib::templates::Template;

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

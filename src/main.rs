#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use(get, routes, catch, catchers)]
extern crate rocket;
#[macro_use(Serialize)]
extern crate serde_derive;
extern crate rocket_contrib;
extern crate serde_json;
extern crate grass;
extern crate notify;

use rocket::Rocket;
use rocket_contrib::templates::Template;
use scss::SCSSLoader;

mod api;
mod static_files;
mod errors;
mod templates;
mod scss;

fn main() {
    rocket().launch();
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/api", api::routes())
        .mount("/static", static_files::routes())
        .mount("/", templates::routes())
        .register(errors::catchers())
        .attach(Template::fairing())
        .attach(SCSSLoader::new())
}

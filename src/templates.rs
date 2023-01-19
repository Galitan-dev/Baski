use rocket::Route;
use rocket_contrib::templates::{Template, tera::Context};

pub fn routes() -> Vec<Route> {
    routes![index, hello]
}

#[get("/")]
fn index() -> Template {
    Template::render("index", Context::new())
}

#[get("/hello/<name>")]
fn hello(name: String) -> Template {
    Template::render("hello", GreetingContext { name })
}

#[derive(Serialize)]
struct GreetingContext {
    name: String
}

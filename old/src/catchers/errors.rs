use rocket::{Catcher, Request};
use rocket_contrib::templates::{tera::Context, Template};

pub fn catchers() -> Vec<Catcher> {
    catchers![not_found]
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut ctx = Context::new();
    ctx.insert("path", req.uri().path());
    Template::render("error/404", &ctx)
}

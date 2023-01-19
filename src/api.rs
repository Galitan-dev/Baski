use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![index]
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

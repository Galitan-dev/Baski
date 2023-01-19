use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![index]
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

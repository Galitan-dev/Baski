use poem::{get, handler, web::Path, Route};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

pub fn routes() -> Route {
    Route::new().at("/hello/:name", get(hello))
}

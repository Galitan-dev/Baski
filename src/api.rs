use poem::{get, handler, web::Path, IntoEndpoint, Route};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

pub fn endpoint() -> impl IntoEndpoint {
    Route::new().at("/hello/:name", get(hello))
}

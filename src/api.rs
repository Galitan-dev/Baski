use poem::{Route, handler, web::Path, get};

#[handler]
fn hello(Path(name): Path<String>) -> String {
	format!("hello: {}", name)
}

pub fn api() -> Route {
    Route::new().at("/hello/:name", get(hello))
}
use poem::{
    error::InternalServerError,
    get, handler,
    web::{Html, Path},
    Route,
};
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("web/templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {e}");
                ::std::process::exit(1);
            }
        }
    };
}

#[handler]
pub fn home() -> Result<Html<String>, poem::Error> {
    TEMPLATES
        .render("home.html", &Context::new())
        .map_err(InternalServerError)
        .map(Html)
}

#[handler]
pub fn hello(Path(name): Path<String>) -> Result<Html<String>, poem::Error> {
    let mut ctx = Context::new();
    ctx.insert("name", &name);
    TEMPLATES
        .render("hello.html", &ctx)
        .map_err(InternalServerError)
        .map(Html)
}

pub fn routes() -> Route {
    Route::new()
        .at("/", get(home))
        .at("/hello/:name", get(hello))
}

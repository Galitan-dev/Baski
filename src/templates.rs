use poem::{
    error::{InternalServerError, NotFoundError},
    get, handler,
    http::StatusCode,
    web::{Html, Path},
    EndpointExt, IntoEndpoint, Response, Route,
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
fn home() -> Result<Html<String>, poem::Error> {
    TEMPLATES
        .render("home.html", &Context::new())
        .map_err(InternalServerError)
        .map(Html)
}

#[handler]
fn hello(Path(name): Path<String>) -> Result<Html<String>, poem::Error> {
    let mut ctx = Context::new();
    ctx.insert("name", &name);
    TEMPLATES
        .render("hello.html", &ctx)
        .map_err(InternalServerError)
        .map(Html)
}

async fn not_found(_: NotFoundError) -> Response {
    match TEMPLATES.render("error/404.html", &Context::new()) {
        Ok(html) => Response::builder().status(StatusCode::NOT_FOUND).body(html),
        Err(err) => InternalServerError(err).into_response(),
    }
}

pub fn endpoint() -> impl IntoEndpoint {
    Route::new()
        .at("/", get(home))
        .at("/hello/:name", get(hello))
        .catch_error(not_found)
        .into_endpoint()
}

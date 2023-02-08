use std::sync::Mutex;

use i18n::TranslateFilter;
use poem::{
    error::{InternalServerError, NotFoundError},
    get, handler,
    http::StatusCode,
    i18n::Locale,
    web::{Html, Path},
    EndpointExt, IntoEndpoint, Response, Route,
};
use tera::{Context, Tera};

use crate::live_reloading::attach_live_reloading;

mod i18n;

lazy_static! {
    pub static ref TEMPLATES: Mutex<Tera> = {
        Mutex::new(match Tera::new("web/templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {e}");
                ::std::process::exit(1);
            }
        })
    };
}

#[handler]
fn home(locale: Locale) -> Result<Html<String>, poem::Error> {
    render("home.html", Context::new(), locale)
}

#[handler]
fn hello(locale: Locale, Path(name): Path<String>) -> Result<Html<String>, poem::Error> {
    let mut ctx = Context::new();
    ctx.insert("name", &name);

    render("hello.html", ctx, locale)
}

fn render(template: &'static str, ctx: Context, locale: Locale) -> Result<Html<String>, poem::Error> {
    let mut tera = TEMPLATES.lock().unwrap().clone();

    tera.register_filter("translate", TranslateFilter::make_for(locale));

    tera.render(template, &ctx)
        .map_err(|err| {
            println!("{err:#?}");
            InternalServerError(err)
        })
        .map(Html)
}

async fn not_found(_: NotFoundError) -> Response {
    match TEMPLATES
        .lock()
        .unwrap()
        .render("error/404.html", &Context::new())
    {
        Ok(html) => Response::builder().status(StatusCode::NOT_FOUND).body(html),
        Err(err) => InternalServerError(err).into_response(),
    }
}

pub fn endpoint() -> impl IntoEndpoint {
    Route::new()
        .at("/", get(home))
        .at("/hello/:name", get(hello))
        .catch_error(not_found)
        .after(attach_live_reloading)
}

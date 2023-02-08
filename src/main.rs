use config::{Config, LogLevel};
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};
use tracing_subscriber::{fmt::format, prelude::__tracing_subscriber_field_MakeExt};

mod api;
mod config;
mod live_reloading;
mod loaders;
mod templates;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = "baski.toml".into();
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if CONFIG.log_level != LogLevel::Error {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt()
        .without_time()
        .fmt_fields(
            format::debug_fn(|writer, _, value| write!(writer, "{:?}", value)).delimited(" "),
        )
        .init();

    loaders::load()?;

    let mut app = Route::new()
        .nest("/", templates::endpoint())
        .nest("/api", api::endpoint())
        .nest("/static", StaticFilesEndpoint::new("static"));

    if CONFIG.live_reloading {
        app = app.nest("/live_reloading", live_reloading::endpoint());
    }

    Server::new(TcpListener::bind(format!(
        "{}:{}",
        CONFIG.hostname, CONFIG.port
    )))
    .run(app)
    .await
}

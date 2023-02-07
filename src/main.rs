use config::{Config, LogLevel};
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};

mod api;
mod config;
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
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .nest("/", templates::endpoint())
        .nest("/api", api::endpoint())
        .nest("/static", StaticFilesEndpoint::new("static"));

    Server::new(TcpListener::bind(format!(
        "{}:{}",
        CONFIG.hostname, CONFIG.port
    )))
    .run(app)
    .await
}

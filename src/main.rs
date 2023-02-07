use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};

mod api;
mod templates;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .nest("/", templates::routes())
        .nest("/api", api::routes())
        .nest("/static", StaticFilesEndpoint::new("static"));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}

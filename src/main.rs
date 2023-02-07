use poem::{listener::TcpListener, Route, Server, endpoint::StaticFilesEndpoint};

mod api;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

	let app = Route::new()
		.nest("/api", api::api())
		.nest("/static", StaticFilesEndpoint::new("static"),
    );
	
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
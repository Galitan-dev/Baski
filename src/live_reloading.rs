use std::{
    sync::{mpsc, Mutex},
    time::Duration,
};

use colored::Colorize;
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use poem::{
    get, handler,
    web::websocket::{Message, WebSocket, WebSocketStream},
    IntoEndpoint, IntoResponse, Response, Route,
};

use crate::{templates::TEMPLATES, CONFIG};

lazy_static! {
    static ref SINKS: Mutex<Vec<SplitSink<WebSocketStream, Message>>> = Mutex::new(Vec::new());
}

#[handler]
fn ws(ws: WebSocket) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut _stream) = socket.split();

        SINKS.lock().unwrap().push(sink);
    })
}

pub fn endpoint() -> impl IntoEndpoint {
    tokio::spawn(async {
        let (wtx, wrx) = mpsc::channel();

        let mut watcher: RecommendedWatcher =
            notify::watcher(wtx, Duration::from_millis(500)).unwrap();
        watcher.watch("web", RecursiveMode::Recursive).unwrap();

        println!(
            "{} {} {}{}",
            "⚡️ Live Reloading for".yellow(),
            "browser".white().bold(),
            "enabled".green(),
            "!".yellow()
        );

        loop {
            let res = wrx.recv();
            match res.unwrap() {
                DebouncedEvent::Create(_)
                | DebouncedEvent::Write(_)
                | DebouncedEvent::Rename(_, _)
                | DebouncedEvent::Remove(_) => {
                    TEMPLATES.lock().unwrap().full_reload().unwrap();
                    let len = SINKS.lock().unwrap().len();

                    for _ in 0..len {
                        let opt = SINKS.lock().unwrap().pop();
                        if let Some(mut sink) = opt {
                            sink.send(Message::Binary(Vec::new())).await.unwrap();
                        }
                    }
                }
                _ => (),
            }
        }
    });

    Route::new().at("/subscribe", get(ws))
}

pub async fn attach_live_reloading(res: poem::Result<Response>) -> poem::Result<Response> {
    if CONFIG.live_reloading {
        match res {
            Ok(mut resp) => {
                let html = resp.take_body().into_string().await.unwrap();
                let html = html.replace(
                    "</body>",
                    r#"<script src="/static/js/live_reloading.js"></script></body>"#,
                );
                resp.set_body(html);
                Ok(resp)
            }
            Err(err) => Err(err),
        }
    } else {
        res
    }
}

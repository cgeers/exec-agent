#![deny(warnings)]
use futures_util::{FutureExt, StreamExt};
use warp::Filter;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/web/ui"]
struct WebUI;

#[tokio::main]
pub async fn start() {

    let webui = warp::path("ui").and(warp_embed::embed(&WebUI));
    let websocket = warp::path("api")
      .and(warp::path("ws"))
      .and(warp::ws())
      .map(|ws: warp::ws::Ws| {
          ws.on_upgrade(|websocket| {
              // Just echo all messages back...
              let (tx, rx) = websocket.split();
              rx.forward(tx).map(|result| {
                  if let Err(e) = result {
                      log::error!("websocket error: {:?}", e);
                  }
              })
          })
      });

    let routes = webui.or(websocket);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
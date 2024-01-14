use tungstenite::{connect, Message};
use url::Url;
use log;

pub fn websocket() {
  let (mut socket, response) =
        connect(Url::parse("ws://localhost:3012/socket").unwrap()).expect("Can't connect");

    log::debug!("Connected to the server");
    log::debug!("Response HTTP code: {}", response.status());
    log::debug!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        log::debug!("* {}", header);
    }

    socket.send(Message::Text("Hello WebSocket".into())).unwrap();
    loop {
        let msg = socket.read().expect("Error reading message");
        log::info!("Received from upstream: {}", msg);
    }
    // socket.close(None);
}
use tungstenite::{connect, Message};
use url::Url;
use std::thread;
use log;

mod web;

fn main() {
    pretty_env_logger::init();

    thread::Builder::new().name("web".to_string()).spawn( || {
        web::server::start();
    }).ok();


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

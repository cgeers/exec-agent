use std::thread;

mod web;
mod control;

fn main() {
    pretty_env_logger::init();

    let webserver = thread::Builder::new().name("web".to_string()).spawn( || {
        web::server::start();
    }).expect("web server start failed");

    thread::Builder::new().name("control".to_string()).spawn( || {
        control::client::websocket();
    }).expect("control connection failed");
    
    webserver.join().expect("web server stopped")
}

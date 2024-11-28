use crate::proto_core as core;
use protobuf::Message;
use std::io::{BufWriter, Write};
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time;

fn handle_new_connection(stream: TcpStream, _addr: SocketAddr) {
    log::info!("Handling tcp stream ");
    let mut writer = BufWriter::new(&stream);
    let second = time::Duration::from_secs(1);
    loop {
        let book = core::core::return_addressbook();
        match Message::write_to_writer(&book, &mut writer) {
            Ok(_) => {
                let _ = writer.flush();
            }
            Err(e) => log::info!("Failed to convert to bytes: Err ({})", e),
        };
        thread::sleep(second);
    }
}
pub fn server(ip_addr: String, port: u16) {
    log::info!("Starting server on {}:{}", ip_addr, port);
    let tcp_listener = match TcpListener::bind(format!("{}:{}", ip_addr, port)) {
        Ok(listener) => listener,
        Err(e) => panic!("Failed to bind: Err ({})", e),
    };

    loop {
        match tcp_listener.accept() {
            Ok((stream, addr)) => {
                log::info!("Accepting connection from {}", addr);
                thread::spawn(move || {
                    log::info!("creating new thread to handle tcp stream...");
                    handle_new_connection(stream, addr);
                });
            }
            Err(e) => log::info!("Failed to accept: Err ({})", e),
        };
    }
}

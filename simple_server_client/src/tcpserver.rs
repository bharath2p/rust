use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn handle_new_connection(tcp_stream: TcpStream) {
    log::info!("Handling tcp stream ");
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
                thread::spawn(|| {
                    log::info!("creating new thread to handle tcp stream...");
                    handle_new_connection(stream);
                });
            }
            Err(e) => log::info!("Failed to accept: Err ({})", e),
        };
    }
}

use std::io::{BufRead, BufReader};
use std::net::TcpStream;

fn handle_connection(stream: TcpStream) {
    log::info!("Handling the connection...");
    let mut reader = BufReader::new(&stream);
    loop {
        let mut text = String::new();
        match reader.read_line(&mut text) {
            Ok(result) => {
                log::info!("Read: len = {}, data = {}", result, text);
            }
            Err(e) => {
                log::info!("Read failed. Err({})", e);
            }
        }
    }
}
pub fn client(ip_addr: String, port: u16) {
    log::info!("Establishing connection to {}:{}", ip_addr, port);
    match TcpStream::connect(format!("{}:{}", ip_addr, port)) {
        Ok(stream) => {
            log::info!("Connection established...");
            handle_connection(stream);
        }
        Err(e) => panic!("Connect failed. Err ({})", e),
    }
}

use std::net::TcpStream;

fn handle_connection(stream: TcpStream) {
    log::info!("I am on!")
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

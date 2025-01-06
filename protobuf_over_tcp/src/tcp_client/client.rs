use crate::protos;
use crate::Phonebook;
use std::io::BufReader;
use std::io::Read;
use std::net::TcpStream;

use protobuf::Message;
use protos::addressbook::AddressBook;

fn handle_connection(stream: TcpStream) {
    log::info!("Handling the connection...");
    let mut reader = BufReader::new(stream);
    loop {
        /* hardcoding protobuf size as 63*/
        let mut buff = vec![0; 63];
        if reader.read_exact(&mut buff).is_err() {
            log::info!("Error reading message");
            break;
        }

        match AddressBook::parse_from_bytes(&buff[..]) {
            Ok(book) => {
                log::info!("read book = {}", book);
            }
            Err(e) => {
                log::info!("Unable to parse bytes. Err({})", e);
            }
        }
    }
}
pub fn client(ip_addr: String, port: u16, book: Phonebook) {
    log::info!(
        "Establishing connection to {}:{} to publish {:?}",
        ip_addr,
        port,
        book
    );
    match TcpStream::connect(format!("{}:{}", ip_addr, port)) {
        Ok(stream) => {
            log::info!("Connection established...");
            handle_connection(stream);
        }
        Err(e) => panic!("Connect failed. Err ({})", e),
    }
}

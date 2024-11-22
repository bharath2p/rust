mod cliparser;
mod tcpclient;
mod tcpserver;

fn main() {
    cliparser::cli_parser(tcpserver::server, tcpclient::client);
    println!("Hello, world!");
}

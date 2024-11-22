mod cliparser;
mod tcpclient;
mod tcpserver;

fn main() {
    femme::start();
    cliparser::cli_parser(tcpserver::server, tcpclient::client);
}

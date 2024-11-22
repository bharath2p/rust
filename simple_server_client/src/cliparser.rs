use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(about = "Simple TCP server client. One server and multiple clients can be spawned")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Server {
        // IP address to bind the server
        ip_addr: String,
        // Port address to bind the server
        port: u16,
    },
    Client {
        // Destination IP address
        ip_addr: String,
        // Destination port
        port: u16,
    },
}

pub fn cli_parser<Func1, Func2>(server: Func1, client: Func2)
where
    Func1: Fn(String, u16),
    Func2: Fn(String, u16),
{
    let cli: Cli = Cli::parse();
    match cli.command {
        Commands::Client { ip_addr, port } => client(ip_addr, port),
        Commands::Server { ip_addr, port } => server(ip_addr, port),
    }
}

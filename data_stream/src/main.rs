mod protos;
use clap::{Parser, Subcommand};
use protobuf::Message;
use protos::stats::Statistics;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::exit;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(about = "Data stream example...")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ConfigFile { path: String },
}

#[derive(Deserialize)]
struct Data {
    log: Log,
    server: Option<Server>,
    client: Option<Client>,
}
#[derive(Deserialize)]
struct Log {
    level: String,
    log_type: String,
    log_file: String,
}
#[derive(Deserialize)]
struct Server {
    host: String,
    port: u16,
}
#[derive(Deserialize)]
struct Client {
    host: String,
    port: u16,
}

impl Data {
    fn Display() {}
}

fn read_file(path: String) -> Option<Data> {
    log::info!("Reading file {}", path);
    if !Path::new(&path).exists() {
        log::info!("{} File does not exists...", path);
        return None;
    }
    let contents = fs::read_to_string(&path).expect("Reading file..");
    log::info!("contents {}", contents);
    let data: Data = toml::from_str(&contents).expect("conversion should succeed");
    Some(data)
}

fn main() {
    /*
    let msg = Statistics::new();
    println!("msg = {}, size = {}", msg, Message::compute_size(&msg));
    let buff = vec![0; Message::compute_size(&msg).try_into().unwrap()];
    */
    femme::start();
    log::info!("Starting application...");
    let cli: Cli = Cli::parse();
    match cli.command {
        Commands::ConfigFile { path } => {
            let data = read_file(path);
        }
    }
}

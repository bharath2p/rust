mod protos;
use clap::{Parser, Subcommand};
use protobuf::Message;
use protos::stats::Statistics;
use std::path::Path;

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

fn read_file(path: String) {
    log::info!("Reading file {}", path);
    if !Path::new(&path).exists() {
        log::info!("{} File does not exists...", path);
        return;
    } else {
        log::info!("{} File exists...", path);
    }
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
        Commands::ConfigFile { path } => read_file(path),
    }
}

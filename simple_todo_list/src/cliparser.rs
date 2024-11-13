use clap::{Parser, Subcommand};

#[derive(Parser)]
//#[command(author, version, about, long_about = None)]
#[command(version, about, long_about = None)]
#[command(about = "Tool to maintain the todos. I will add more feature here 
    but right now its empty, with some trial and errors")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Create {
        task: String,
        details: String,
    },
    Update {
        task: String,
    },
}

pub fn cli_parser<F>(callback: F)
where
    F: Fn(String, String),
{
    let cli = Cli::parse();
    match cli.command {
        Commands::Create { task, details } => callback(task, details),
        Commands::Update { task } => println!("update task = {}", task),
    }
}

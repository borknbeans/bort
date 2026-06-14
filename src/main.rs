use std::time::SystemTime;

use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Prompt { dir, terminal_width } => prompt(dir, terminal_width),
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Prompt {
        #[arg(long)]
        dir: String,

        #[arg(long)]
        terminal_width: usize,
    },
}

fn prompt(dir: String, terminal_width: usize) {
    let total_len = dir.len();
    let width = terminal_width - total_len;

    let system_time = SystemTime::now();
    let datetime: DateTime<Local> = system_time.into();
    println!("{}{:>width$}\n> ", dir, datetime.format("%H:%M")); 
    // println!("%~\n> ");
    // println!("{}> ", dir); 
}

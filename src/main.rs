use std::time::SystemTime;

use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Prompt { right, dir } => {
                if right {
                    prompt_right();
                } else {
                    prompt(dir.unwrap_or_default());
                }
            },
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
        #[arg(long, default_value_t = false)]
        right: bool,

        #[arg(long)]
        dir: Option<String>,        
    },
}

fn prompt(dir: String) {
    println!("{}> ", dir); 
}

fn prompt_right() {
    let system_time = SystemTime::now();
    let datetime: DateTime<Local> = system_time.into();

    println!("{}", datetime.format("%T"));
}

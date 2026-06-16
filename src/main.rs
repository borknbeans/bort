use std::time::SystemTime;

use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};

use crate::modules::{Module, dir::Dir};


mod config;
mod modules;

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Prompt { home_dir, dir, terminal_width } => prompt(home_dir, dir, terminal_width),
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
        home_dir: String,

        #[arg(long)]
        dir: String,

        #[arg(long)]
        terminal_width: usize,
    },
}

fn prompt(home_dir: String, mut dir: String, terminal_width: usize) {
    let config = config::parse_config(&home_dir);

    // Replace home directory with ~ if applicable
    // if dir.starts_with(&home_dir) {
    //     dir = dir.replace(&home_dir, "~")
    // }

    let mut prompt = config.format.clone();

    let dir_module: Dir = Dir { home_dir, full_dir: dir };
    prompt = prompt.replace(&dir_module.module_name(), &dir_module.format());


    println!("{}", prompt);

    // let total_len = dir.len();
    // let width = terminal_width - total_len;

    // let system_time = SystemTime::now();
    // let datetime: DateTime<Local> = system_time.into();
    // println!("%F{{#689d6a}}{}%f{:>width$}\n> ", dir, datetime.format("%H:%M")); 
}

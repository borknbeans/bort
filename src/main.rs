
use clap::{Parser, Subcommand, Args};

mod config;
mod modules;

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Prompt(prompt_args) => prompt(prompt_args),
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
    Prompt(PromptArgs),
}


#[derive(Args)]
pub(crate) struct PromptArgs {
    #[arg(long)]
    home_dir: String,

    #[arg(long)]
    curr_dir: String,

    #[arg(long)]
    terminal_width: usize,
}

fn prompt(prompt_args: PromptArgs) {
    let config = config::parse_config(&prompt_args.home_dir);

    println!("{}", modules::format_modules(prompt_args, config));
    // println!("%F{{#689d6a}}{}%f{:>width$}\n> ", dir, datetime.format("%H:%M")); 
}

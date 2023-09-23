mod args;
mod get_curl;
mod types;
mod utils;

use std::process;

use clap::{Parser, Subcommand};
//use types::curl::Curl;//TODO

#[derive(Parser)]
#[command(author, version)] // from Cargo.toml
struct Cli {
    ///Copy a command's output to clipboard
    #[arg(short, long, default_value_t = false)]
    copy: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///Interactive UI to choose a request from current directory and get it's curl
    Curl,
}

fn main() {
    let pocc = Cli::parse();
    let result = match pocc {
        Cli {
            command: Commands::Curl,
            ..
        } => get_curl::get_curl(),
    };

    match (pocc, result) {
        (Cli {copy: true, ..}, Ok(result)) => {
            utils::copy_to_clipbaord(result).expect("failed to copy to clipboard");
        },
        (Cli {copy: false, ..}, Ok(result)) => {
            println!("{result}");
        },
        (_, Err(result)) => {
            eprintln!("{result}");
            process::exit(1)
        }
    }
}

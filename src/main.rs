mod args;
mod types;
mod utils;
mod get_curl;

use clap::{Parser, Subcommand};
//use types::curl::Curl;//TODO

#[derive(Parser)]
#[command(author, version)] // from Cargo.toml
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///gets curl of a request out of collections in current directory
    GetCurl,
}

fn main() {
    let pocc = Cli::parse();
    match pocc {
        Cli { command: Commands::GetCurl, ..} => {
            get_curl::get_curl();
        },
    }
}

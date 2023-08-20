mod args;
mod types;
mod utils;
mod get_curl;

use clap::{Parser, Subcommand};
//use types::curl::Curl;//TODO

#[derive(Parser)]
#[command(author, version)] // from Cargo.toml
struct Cli {
    ///get curl of the request
    #[arg(short, long, value_name = "collection/folder/../request_name")]
    curl: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///gets curl of a request out of collections
    GetCurl,
}

fn main() {
    let pocc = Cli::parse();

    match pocc {
        Cli{ curl: Some(path),.. } => {
            let _ = utils::get_req_from_current_dir(&path.split("/").collect());
            println!("alloo");
        },
        Cli { command: Commands::GetCurl, ..} => {
            println!("GetCurl command called");
        },
//        _ => println!("print --help output here"),

    }
}

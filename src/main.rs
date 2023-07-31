mod args;
mod types;
mod utils;

use clap::Parser;
use types::curl::Curl;

#[derive(Parser)]
#[command(author, version)] // from Cargo.toml
struct Cli {
    ///get curl of the request
    #[arg(short, long, value_name = "collection/folder/../request_name")]
    curl: String,
}

fn main() {
    let pocc = Cli::parse();

    match utils::get_req_from_current_dir(&pocc.curl.split("/").collect()) {
        Ok(req) => println!("{}", Curl(req)),
        Err(e) => println!("{e}"),
    }
}

mod args;
mod types;
mod utils;

use clap::Parser;
use std::fs;
use types::postman::Collection;

#[derive(Parser)]
#[command(author, version)] // from Cargo.toml
struct Cli {
    ///get curl of the request
    #[arg(short, long, value_name = "collection/folder/../request_name")]
    curl: String,
}

#[tokio::main]
async fn main() {
    let pocc = Cli::parse();

    //****************
    let file_path = "../../TestCollection.postman_collection.json";
    let file_content = fs::read_to_string(file_path).unwrap();
    let collection: Collection = serde_json::from_str(file_content.as_str())
        .expect("Unable to parse collection {file_path}");
    //****************

    collection.get_curl(pocc.curl)
}

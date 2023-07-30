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
    old_curl: Option<String>,
    ///process the request and return the response
    #[arg(short, long, value_name = "collection/folder/../request_name")]
    process: Option<String>,
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
    match pocc {
        Cli {
            old_curl: Some(path), ..
        } => collection.get_old_curl(path),
        Cli {
            process: Some(path),
            ..
        } => collection.direct(path).await,
        _ => println!("give all the requests found"),
    }
}

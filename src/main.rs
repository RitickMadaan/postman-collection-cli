mod types;

use clap::Parser;
use std::fs;
use types::Collection;

#[derive(Parser)]
#[command(author, version)] // from Cargo.toml
struct Cli {
    ///get curl from a request
    #[arg(short, long, value_name = "collection/folder/../request_name")]
    curl: Option<String>,
    ///run a request
    #[arg(short, long, value_name = "collection/folder/../request_name")]
    direct: Option<String>,
}

fn main() {
    let pocc = Cli::parse();
    //****************
    let file_path = "../../TestCollection.postman_collection.json";
    let file_content = fs::read_to_string(file_path).unwrap();
    let _collection: Collection = serde_json::from_str(file_content.as_str())
        .expect("Unable to parse collection {file_path}");

    //****************
    //
    match pocc {
        Cli { curl: Some(c), .. } => println!("give curl here for {c}"),
        Cli {
            direct: Some(d), ..
        } => println!("give response from the request run for {d}"),
        _ => println!("give all the requests found"),
    }
}

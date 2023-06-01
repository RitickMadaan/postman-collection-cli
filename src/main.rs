mod types;

use std::fs;

use clap::Parser;
use types::Collection;

#[derive(Parser)]
#[command(author, version)] // from Cargo.toml
struct Cli {
}

fn main() {
    let pocc = Cli::parse();
    //****************
    //let file_path = "./TestCollection.postman_collection.json";
    let file_path = "./tests.json";
    let file_content = fs::read_to_string(file_path).unwrap();
    let collection: Result<Collection, serde_json::Error> = serde_json::from_str(file_content.as_str());
    match collection {
        Err(e) => println!("{e}"),
        Ok(_) => println!("parsing successful"),
    }
    //****************
    match pocc {
        _ => println!("Welcome to pocc!\n")
    }
}

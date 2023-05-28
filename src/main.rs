mod types;

use clap::Parser;

#[derive(Parser)]
#[command(author, version)] // from Cargo.toml
struct Cli {
}

fn main() {
    let pocc = Cli::parse();
    match pocc {
        _ => println!("Welcome to pocc!")
    }
}

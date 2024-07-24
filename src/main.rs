use bcrypt::{DEFAULT_COST, hash};
use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    value: String
}

fn main() {
    let args = Args::parse();
    let hashed = hash(&args.value, DEFAULT_COST).unwrap();

    println!("{}", &hashed)
}
use anyhow::Result;
use clap::Parser;
use cli::Cli;
// use live::cli;
mod cli;

fn main() {
    let cli = Cli::parse();
    println!("Hello, world!");
    println!(
        "Input: name={},live={},room_number={}",
        cli.name, cli.live, cli.room_number
    );
}

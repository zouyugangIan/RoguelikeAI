use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    pub name: String,
    pub live: String,
    pub room_number: usize,
}

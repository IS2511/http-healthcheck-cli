use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The URL to check for HTTP 200
    pub url: String,

    /// Request interval in seconds
    #[arg(short, long)]
    pub interval: u32,
}

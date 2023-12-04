use clap::Parser;
use std::{error::Error, time::Duration};

mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();

    let url = reqwest::Url::parse(args.url.as_str());

    if url.is_err() {
        println!("URL parsing error");
    }
    let url = url?;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(args.timeout))
        .build()?;

    let mut interval = tokio::time::interval(Duration::from_secs(args.interval));

    loop {
        let url = url.clone();

        interval.tick().await;

        let response = client.get(url.as_ref())
            .header(reqwest::header::USER_AGENT, 
                format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")))
            .send().await;

        let result = match response {
            Ok(response) => {
                if response.status().is_success() {
                    format!("OK({})", response.status().as_u16())
                } else {
                    format!("ERR({})", response.status().as_u16())
                }
            },
            Err(err) => {
                format!("ERR: {}", err)
            },
        };

        println!("Checking '{url}'. Result: {result}");
    }
}

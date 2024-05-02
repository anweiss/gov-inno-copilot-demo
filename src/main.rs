use std::num::NonZeroU32;

use artem::{self, config::ConfigBuilder};
use clap::{Parser, Subcommand};
use image::open;
use reqwest;
use tokio;

#[derive(Parser)]
#[command(
    name = "govpilot",
    about = "A CLI for government innovation.",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello,
    Joke,
    Fact,
    Art {
        #[arg(short, long)]
        path: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => hello(),
        Commands::Joke => joke().await?,
        Commands::Fact => fact(),
        Commands::Art { path } => art(&path)?,
    }

    Ok(())
}

fn hello() {
    println!("Hello from govpilot!");
}

async fn joke() -> Result<(), reqwest::Error> {
    let response = reqwest::Client::new()
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("{}", response["joke"].to_string());

    Ok(())
}

fn fact() {
    println!("Did you know? Here's a government fact.");
}

fn art(path: &str) -> Result<(), &'static str> {
    let image = match open(path) {
        Ok(img) => img,
        Err(e) => {
            println!("Failed to open image: {}", e);
            return Ok(());
        }
    };
    let ascii_art = artem::convert(
        image,
        &ConfigBuilder::new()
            .target_size(NonZeroU32::new(250).ok_or("invalid size")?)
            .build(),
    );
    println!("{}", ascii_art);

    Ok(())
}

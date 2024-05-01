use clap::{Parser, Subcommand};
use reqwest;
use serde_json;
use std::io;
use tokio;

#[derive(Parser)]
#[command(name = "govpilot", about = "A simple CLI application for demonstration purposes.", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Outputs a simple greeting
    Hello,
    /// Tells a simple joke
    Joke,
    /// Shares a simple fact
    Fact,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => {
            println!("Please enter your name:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            println!("Hello {}! You're about to experience the coolest CLI app ever created!", input.trim());
        },
        Commands::Joke => {
            match fetch_joke().await {
                Ok(joke) => println!("{}", joke),
                Err(e) => println!("Failed to fetch joke: {}", e),
            }
        },
        Commands::Fact => println!("Did you know? Rust was named the 'most loved programming language' in the Stack Overflow Developer Survey five years in a row."),
    }
}

async fn fetch_joke() -> Result<String, reqwest::Error> {
    let response = reqwest::Client::new()
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response["joke"].to_string())
}

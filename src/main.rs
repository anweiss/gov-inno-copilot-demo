use clap::{Parser, Subcommand};
use reqwest;
use tokio;

#[derive(Parser)]
#[command(name = "govpilot", about = "A CLI for government innovation.", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello,
    Joke,
    Fact,
    Art,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => hello(),
        Commands::Joke => joke().await,
        Commands::Fact => fact(),
        Commands::Art => art(),
    }
}

fn hello() {
    println!("Hello from govpilot!");
}

async fn joke() {
    let url = "https://icanhazdadjoke.com/";
    let client = reqwest::Client::new();
    let res = client.get(url)
        .header("Accept", "application/json")
        .send()
        .await
        .expect("Failed to send request");

    if res.status().is_success() {
        let joke = res.json::<Joke>().await.expect("Failed to parse joke");
        println!("{}", joke.joke);
    } else {
        println!("Failed to fetch joke");
    }
}

fn fact() {
    println!("Did you know? Here's a government fact.");
}

fn art() {
    println!("Displaying government art.");
}

#[derive(serde::Deserialize)]
struct Joke {
    joke: String,
}

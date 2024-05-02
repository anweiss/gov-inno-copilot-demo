use clap::{Parser, Subcommand, Arg};
use reqwest;
use tokio;
use artem::{self, config::ConfigBuilder};
use image::open;

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
    Art {
        #[arg(short, long)]
        path: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => hello(),
        Commands::Joke => joke().await,
        Commands::Fact => fact(),
        Commands::Art { path } => art(&path).await,
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

async fn art(path: &str) {
    let image = match open(path) {
        Ok(img) => img,
        Err(e) => {
            println!("Failed to open image: {}", e);
            return;
        }
    };
    let ascii_art = artem::convert(image, &ConfigBuilder::new().build());
    println!("{}", ascii_art);
}

#[derive(serde::Deserialize)]
struct Joke {
    joke: String,
}

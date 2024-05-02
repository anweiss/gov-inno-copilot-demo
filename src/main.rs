use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => hello(),
        Commands::Joke => joke(),
        Commands::Fact => fact(),
        Commands::Art => art(),
    }
}

fn hello() {
    println!("Hello from govpilot!");
}

fn joke() {
    println!("Here's a government joke for you!");
}

fn fact() {
    println!("Did you know? Here's a government fact.");
}

fn art() {
    println!("Displaying government art.");
}

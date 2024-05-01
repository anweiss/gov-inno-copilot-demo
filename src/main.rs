use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => println!("Hello! Welcome to govpilot."),
        Commands::Joke => println!("Why did the programmer quit his job? Because he didn't get arrays."),
        Commands::Fact => println!("Did you know? Rust was named the 'most loved programming language' in the Stack Overflow Developer Survey five years in a row."),
    }
}

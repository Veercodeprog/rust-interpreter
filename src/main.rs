use clap::{Parser, Subcommand};
use codecrafters_interpreter::*;
use miette::{IntoDiagnostic, WrapErr};
use std::env;
use std::fs;
use std::path::PathBuf; // Add this line

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// does testing things
    Tokenize {
        /// lists test values
        filename: PathBuf,
    },
}

fn main() -> miette::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Tokenize { filename } => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            //into_didactic() is a method that converts an error into a diagnostic error i.e non - minimal
            //error to mieete error

            let file_contents = fs::read_to_string(&filename)
                .into_diagnostic()
                .wrap_err_with(|| format!("reading '{}' failed ", filename.display()))?;
            for token in Lexer::new(&file_contents) {
                let token = token?;
                println!("{token} ");
            }
            // Uncomment this block to pass the first stage
        }
    }
    Ok(())
}

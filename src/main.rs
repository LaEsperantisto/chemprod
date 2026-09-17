mod chemistry;
mod cli;

use crate::cli::{dictionary, formula};
use crate::cli::{element, isotope};
use clap::{Parser, Subcommand};
use std::io::{self, Write};

#[derive(Parser)]
#[command(
    name = "chemprod",
    version = "0.1.0",
    about = "Chemistry CLI",
    long_about = "A Chemistry tool for absolutely everything in chemistry: elements, formulae, equations"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Look up an element
    Element {
        /// Element name or symbol
        query: String,
    },
    /// Look up an isotope
    Isotope {
        /// Isotope name or symbol (e.g. carbon-14)
        query: String,
    },
    /// Analyse a chemical formula
    Formula {
        /// Chemical formula (e.g. Ca(OH)2)
        formula: String,
    },
    /// Balance a chemical equation - NOT WORKING
    Equation {
        /// Chemical equation (e.g. H2 + O2 -> H2O)
        equation: String,
    },
    /// Look up terms
    Dictionary {
        /// Term
        term: Vec<String>,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::try_parse();

    match cli {
        Ok(args) => match args.command {
            Some(cmd) => execute_command(cmd),
            None => run_repl(),
        },
        Err(err) => {
            // Handle cases where arguments were provided but invalid
            err.exit();
        }
    }

    Ok(())
}

fn execute_command(command: Commands) {
    match command {
        Commands::Element { query } => {
            element::run(&query);
        }
        Commands::Isotope { query } => {
            isotope::run(&query);
        }
        Commands::Formula { formula } => {
            formula::run(&formula);
        }
        Commands::Equation { equation } => {
            println!("Equation mode");
            println!("Input: {}", equation);
        }
        Commands::Dictionary { term } => {
            dictionary::run(&term.join(" "));
        }
    }
}

fn run_repl() {
    println!("Chemprod REPL mode. Type 'exit' or press Ctrl+C to quit.\n");
    let stdin = io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap() == 0 {
            break; // EOF reached (Ctrl+D)
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line == "exit" || line == "quit" {
            break;
        }

        // Prepend "chemprod" so clap can parse it as full CLI arguments
        let args = std::iter::once("chemprod").chain(line.split_whitespace());

        match Cli::try_parse_from(args) {
            Ok(cli) => {
                if let Some(cmd) = cli.command {
                    execute_command(cmd);
                }
            }
            Err(e) => {
                // Display clap error/help without exiting the loop
                println!("{}", e.render().ansi());
            }
        }
    }
}
use clap::{Parser, Subcommand};

mod cli;
use crate::cli::{element, isotope};

#[derive(Parser)]
#[command(
    name = "chemprod",
    version = "0.0.1",
    about = "Chemistry CLI",
    long_about = "A Chemistry tool for absolutely everything in chemistry: elements, formulae, equations"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
    /// Analyse a chemical formula - NOT WORKING
    Formula {
        /// Chemical formula (e.g. H2O)
        formula: String,
    },
    /// Balance a chemical equation - NOT WORKING
    Equation {
        /// Chemical equation (e.g. H2 + O2 -> H2O)
        equation: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Element { query } => {
            println!("Element mode");
            element::run(query);
        }

        Commands::Isotope { query } => {
            println!("Isotope mode");
            isotope::run(query);
        }

        Commands::Formula { formula } => {
            println!("Formula mode");
            println!("Input: {}", formula);
        }

        Commands::Equation { equation } => {
            println!("Equation mode");
            println!("Input: {}", equation);
        }
    }

    Ok(())
}

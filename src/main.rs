
use std::env;

use crate::cli::element;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    let command = args[1].to_lowercase();

    match command.as_str() {
        "element" => {
            element::run(&args);
        }

        "formula" => {
            if args.len() < 3 {
                eprintln!("Error: missing formula.");
                eprintln!("Usage: chemprod formula <formula>");
                return Ok(());
            }

            let formula = &args[2];

            println!("Formula mode");
            println!("Input: {}", formula);
        }

        "equation" => {
            if args.len() < 3 {
                eprintln!("Error: missing equation.");
                eprintln!("Usage: chemprod equation <equation>");
                return Ok(());
            }

            let equation = &args[2];

            println!("Equation mode");
            println!("Input: {}", equation);
        }

        "help" | "--help" | "-h" => {
            print_help();
        }

        "version" | "--version" | "-V" => {
            println!("chemprod 0.0.1");
        }

        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!();
            print_help();
        }
    }

    Ok(())
}

fn print_help() {
    println!("Chemistry CLI");
    println!();
    println!("Usage:");
    println!("  chemprod element <name|symbol>");
    println!("  chemprod formula <formula>");
    println!("  chemprod equation <equation>");
    println!();
    println!("Commands:");
    println!("  element     Look up an element");
    println!("  formula     Analyse a chemical formula");
    println!("  equation    Balance a chemical equation");
    println!("  help        Show this help message");
    println!("  version     Show the program version");
}

use colored::*;
use tokenizer::error_reporter;
use std::{env::args, path::Path};
pub mod tokenizer;
fn main() {
    // Starting the engine
    println!("{}", "=> Starting Engine...".green().bold());
    
    // Collect command-line arguments
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("{}", "=> Error: No file provided. Usage: engine run <file>".red().bold());
        return;
    }

    // Assume first argument is the command and second is the file path.
    let cmd = &args[1];
    let file = &args[2];

    match cmd.as_str() {
        "run" => {
            println!("{}", format!("=> Attempting to load file at: {}", file).cyan().bold());
            let file_path = Path::new(file);

            // Check if the file exists and is indeed a file.
            if file_path.exists() {
                if file_path.is_file() {
                    println!("{}", "=> File found. Reading file...".cyan().bold());
                    let contents = std::fs::read_to_string(file_path)
                        .expect("=> Error: Failed to read file");
                    println!("{}", "=> File read successfully.".cyan().bold());

                    // Start tokenization.
                    let mut error_reporter = error_reporter::ErrorReporter::new();
                    println!("{}", "=> Tokenizing...".cyan().bold());
                    let tokens = tokenizer::tokenize::tokenize(&contents, &mut error_reporter);
                    println!("{}", format!("=> Tokenization complete. {} tokens generated.", tokens.len()).cyan().bold());

                    // If there are errors, print the detailed report.
                    if error_reporter.has_errors() {
                        println!("{}", "=> Errors encountered during tokenization:".red().bold());
                        error_reporter.print_report(&contents);
                    }

                } else {
                    println!("{}", "=> Error: File exists but is not a regular file.".red().bold());
                }
            } else {
                println!("{}", "=> Error: File does not exist.".red().bold());
            }
        }
        _ => {
            println!("{}", "=> Unknown command. Please use 'run <file>' to start the engine.".red().bold());
        }
    }
}
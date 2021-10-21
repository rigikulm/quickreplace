use text_colorizer::*;
use std::env;
use std::fs;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

/// Prints CLI usage information to the console
/// 
fn print_usage() {
    eprintln!("{} - change occurances of one string into another",
              "quickreplace".yellow());
    eprintln!("Usage: {} <target> <replacement> <INPUT> <OUTPUT>", "quickreplace".yellow());
}

/// Parse the command line arguments and display an error if an incorrect
/// number of parameters are provided.print_usage
fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4 got {}.",
            "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone()
    }
}


fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}' : {:?}",
                "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}' : {:?}",
                "Error:".red().bold(), args.output, e);
            std::process::exit(1);
        }
    };
}

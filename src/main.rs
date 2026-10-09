mod error;
mod lexer;
mod macros;
mod parser;

use error::ToDiagnostics;
use std::{env::args, fs::read_to_string, process::exit};

fn main() {
    // file access
    let path: String = args().skip(1).next().unwrap_or_else(|| {
        eprintln!("File to be parsed was not provided");
        exit(2) // common exit code for command args misusage
    });
    let source: String = read_to_string(&path).unwrap();

    // lexing
    let tokens: Box<[_]> =
        lexer::lex(&source).unwrap_or_else(|errors: Box<[_]>| errors.report(&path, &source));
    println!("{tokens:?}");

    // parsing
}

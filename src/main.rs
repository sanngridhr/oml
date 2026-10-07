mod lexer;

use std::{env::args, fs::read_to_string};

use lexer::lexeme::Lexeme;

fn main() {
    let path: String = args()
        .skip(1)
        .next()
        .unwrap_or_else(|| panic!("File to be parsed was not provided"));
    let code: String = read_to_string(&path).unwrap();

    let tokens: Vec<Lexeme<'_>> =
        lexer::lex(&code, Some("example.oml")).unwrap_or_else(|errors: Box<[String]>| {
            errors.iter().for_each(|e: &String| eprintln!("{e}"));
            eprintln!("{} error(s) found", errors.len());
            std::process::exit(1);
        });
    println!("{tokens:?}");
}

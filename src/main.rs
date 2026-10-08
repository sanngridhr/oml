mod error;
mod lexer;

use std::{env::args, fs::read_to_string, process::exit};

use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFiles,
    term::{
        self, Chars, Config,
        termcolor::{ColorChoice, StandardStream, StandardStreamLock},
    },
};
use error::FileErrors;

fn main() {
    // codespan reporting setup
    let mut files: SimpleFiles<String, String> = SimpleFiles::new();
    let writer: StandardStream = StandardStream::stderr(ColorChoice::Auto);
    let config: Config = Config {
        chars: Chars::ascii(),
        ..Config::default()
    };

    // file access
    let name: String = args()
        .skip(1)
        .next()
        .unwrap_or_else(|| panic!("File to be parsed was not provided"));
    let source: String = read_to_string(&name).unwrap();

    // lexing
    let tokens_id: usize = files.add(name, source.clone());
    let tokens: Vec<_> = lexer::lex(&source).unwrap_or_else(|errors: FileErrors<_>| {
        let mut out: StandardStreamLock<'_> = writer.lock();
        errors
            .to_diagnostics(tokens_id)
            .for_each(|diagnostic: Diagnostic<usize>| {
                term::emit_to_write_style(&mut out, &config, &files, &diagnostic).unwrap()
            });
        exit(1);
    });
    println!("{tokens:?}");
}

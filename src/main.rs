mod parser;

use std::{env::args, fs::read_to_string};

fn main() {
    let path: String = args().skip(1).next()
        .unwrap_or_else(|| panic!("File to be parsed was not provided"));
    let code: String = read_to_string(path).unwrap();

    let parsed = parser::lex(code);
    println!("{:?}", parsed);
}

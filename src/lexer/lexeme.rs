use std::str::Chars;

use crate::lexer::error::LexingErrorKind;

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum Lexeme<'src> {
    // Keywords
    Fun,
    If,
    Then,
    Else,
    In,
    Let,
    Match,
    Rec,

    // Brackets
    LBrace,
    RBrace,
    LParen,
    RParen,
    LSqBra,
    RSqBra,

    // Reserved Ops
    Arrow,
    DoubleArrow,
    Bar,
    Colon,
    Comma,
    EndOfStatement,
    Equals,
    Spread,

    // Literals
    Atom(&'src str),
    Char(char),
    EmptyDict,
    EmptyList,
    Floating(f64),
    Integer(i32),
    String(&'src str),
    Unit,

    // Identifiers
    Identifier(&'src str),
    Operator(&'src str),
    TypeVar(&'src str),
}

pub(super) fn classify(word: &str) -> Result<Lexeme<'_>, LexingErrorKind> {
    let node: Lexeme = match word {
        // Keywords
        "fun" => Lexeme::Fun,
        "if" => Lexeme::If,
        "then" => Lexeme::Then,
        "else" => Lexeme::Else,
        "in" => Lexeme::In,
        "let" => Lexeme::Let,
        "match" => Lexeme::Match,
        "rec" => Lexeme::Rec,

        // Brackets
        "{" => Lexeme::LBrace,
        "}" => Lexeme::RBrace,
        "(" => Lexeme::LParen,
        ")" => Lexeme::RParen,
        "[" => Lexeme::LSqBra,
        "]" => Lexeme::RSqBra,

        // Reserved Ops
        "->" => Lexeme::Arrow,
        "=>" => Lexeme::DoubleArrow,
        "|" => Lexeme::Bar,
        ":" => Lexeme::Colon,
        "," => Lexeme::Comma,
        ";;" => Lexeme::EndOfStatement,
        "=" => Lexeme::Equals,
        ".." => Lexeme::Spread,

        // Literals
        "{}" => Lexeme::EmptyDict,
        "[]" => Lexeme::EmptyList,
        "()" => Lexeme::Unit,
        _ if { is_atom(word) } => Lexeme::Atom(word),
        _ if { is_string(word) } => Lexeme::String(&word[1..word.len() - 1]),
        _ if { is_char(word) } => Lexeme::Char(word.chars().nth(1).unwrap()),
        _ if { is_floating(word) } => Lexeme::Floating(word.parse::<f64>().unwrap()),
        _ if { is_integer(word) } => Lexeme::Integer(
            word.parse::<i32>()
                .map_err(|_| LexingErrorKind::IntegerOverflow)?,
        ),

        // Identifiers
        _ if { is_typevar(word) } => Lexeme::TypeVar(word),
        _ if { is_operator(word) } => Lexeme::Operator(word),
        _ if { is_identifier(word) } => Lexeme::Identifier(word), // should be last

        // Errors
        "''" => Err(LexingErrorKind::EmptyChar)?,
        _ if { word.starts_with('"') } => Err(LexingErrorKind::UnclosedString)?,
        _ => Err(LexingErrorKind::Unrecognised)?, // should be last
    };

    Ok(node)
}

fn is_atom(word: &str) -> bool {
    word.len() > 1 && word.starts_with('#')
}

fn is_string(word: &str) -> bool {
    word.len() >= 2 && word.starts_with('"') && word.ends_with('"')
}

fn is_char(word: &str) -> bool {
    word.chars().count() == 3 && word.starts_with('\'') && word.ends_with('\'')
}

fn is_typevar(word: &str) -> bool {
    word.starts_with('\'') && word.chars().skip(1).all(char::is_alphabetic)
}

fn is_integer(word: &str) -> bool {
    if let Some(tail) = word.strip_prefix('-') {
        return !tail.starts_with('-') && is_integer(tail);
    }

    word.chars().all(|c: char| c.is_ascii_digit())
}

fn is_floating(word: &str) -> bool {
    if let Some(tail) = word.strip_prefix('-') {
        return !tail.starts_with('-') && is_floating(tail);
    }

    match word.split_once('.') {
        Some((int, frac)) => is_integer(int) && is_integer(frac),
        None => false,
    }
}

const OPERATOR_CHARS: &str = "!$%&*+-./:<>=?^|";
fn is_operator(word: &str) -> bool {
    word.chars().all(|c: char| OPERATOR_CHARS.contains(c))
}

fn is_identifier(word: &str) -> bool {
    let mut chars: Chars<'_> = word.chars();

    let Some(first) = chars.next() else {
        return false;
    };

    (first.is_alphabetic() || first == '_')
        && chars.all(|c: char| c.is_alphabetic() || c.is_ascii_digit() || "->".contains(c))
}

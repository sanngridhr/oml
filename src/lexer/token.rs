use regex::Regex;

#[derive(Debug)]
pub(crate) enum ASTNode {
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
    LParen,
    LSqBra,
    RBrace,
    RParen,
    RSqBra,

    // Reserved Ops
    Colon,
    EndOfStatement,
    Equals,
    Bar,
    Arrow,
    DoubleArrow,
    Spread,

    Operator(String),
    Identifier(String),
    Atom(String),
    TypeVar(String),
    Integer(i32),
    Floating(f64),
    String(String),
    Comma,
    EmptyList,
    EmptyDict,
    Unit,
}

const LETTER: &str = "A-Za-zА-ЯҐЄІЇа-яґєії";

pub(crate) fn classify(word: &str) -> ASTNode {
    let re_identifier: Regex = Regex::new(&format!(
        r"^[{LETTER}_](?:[{LETTER}\d_\->]*[{LETTER}\d'])?$"
    ))
    .unwrap();
    let re_atom: Regex = Regex::new(&format!(r"^#[{LETTER}]+$")).unwrap();
    let re_operator: Regex = Regex::new(r"^[!$%&*+\-./:<>=?^|]+$").unwrap();
    let re_typevar: Regex = Regex::new(&format!(r"^'[{LETTER}][{LETTER}\d]*$")).unwrap();
    let re_integer: Regex = Regex::new(r"^-?\d+$").unwrap();
    let re_floating: Regex = Regex::new(r"^-?(?:\d+\.\d*|\.\d+)$").unwrap();
    let re_string: Regex = Regex::new("^\".*\"$").unwrap();

    return match word {
        // Keywords
        "let" => ASTNode::Let,
        "rec" => ASTNode::Rec,
        "in" => ASTNode::In,
        "if" => ASTNode::If,
        "then" => ASTNode::Then,
        "else" => ASTNode::Else,
        "match" => ASTNode::Match,
        "fun" => ASTNode::Fun,

        // Brackets
        "(" => ASTNode::LParen,
        ")" => ASTNode::RParen,
        "{" => ASTNode::LBrace,
        "}" => ASTNode::RBrace,
        "[" => ASTNode::LSqBra,
        "]" => ASTNode::RSqBra,

        // Reserved Ops
        ":" => ASTNode::Colon,
        "=" => ASTNode::Equals,
        ";;" => ASTNode::EndOfStatement,
        "|" => ASTNode::Bar,
        "->" => ASTNode::Arrow,
        "=>" => ASTNode::DoubleArrow,
        ".." => ASTNode::Spread,

        "," => ASTNode::Comma,
        "[]" => ASTNode::EmptyList,
        "{}" => ASTNode::EmptyDict,
        "()" => ASTNode::Unit,
        _ if { re_operator.is_match(word) } => ASTNode::Operator(word.to_owned()),
        _ if { re_integer.is_match(word) } => ASTNode::Integer(word.parse::<i32>().unwrap()),
        _ if { re_floating.is_match(word) } => ASTNode::Floating(word.parse::<f64>().unwrap()),
        _ if { re_identifier.is_match(word) } => ASTNode::Identifier(word.to_owned()),
        _ if { re_atom.is_match(word) } => ASTNode::Atom(word.to_owned()),
        _ if { re_typevar.is_match(word) } => ASTNode::TypeVar(word.to_owned()),
        _ if { re_string.is_match(word) } => ASTNode::String(
            word.strip_prefix("\"")
                .unwrap()
                .strip_suffix("\"")
                .unwrap()
                .to_string(),
        ),
        _ => panic!("Word `{word}` is an invalid token!"),
    };
}

use std::sync::LazyLock;

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
    Atom(String),
    EmptyDict,
    EmptyList,
    Floating(f64),
    Integer(i64),
    String(String),
    Unit,

    // Identifiers
    Identifier(String),
    Operator(String),
    TypeVar(String),

    // Errors
    Unrecognised(String),
}

const LETTER: &str = "A-Za-zА-ЯҐЄІЇа-яґєії";

pub(crate) fn classify(word: &str) -> Result<ASTNode, String> {
    static RE_IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(&format!(
            r"^[{LETTER}_](?:[{LETTER}\d_\->]*[{LETTER}\d'])?$"
        ))
        .unwrap()
    });
    static RE_ATOM: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(&format!(r"^#[{LETTER}]+$")).unwrap());
    static RE_OPERATOR: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^[!$%&*+\-./:<>=?^|]+$").unwrap());
    static RE_TYPEVAR: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(&format!(r"^'[{LETTER}][{LETTER}\d]*$")).unwrap());
    static RE_INTEGER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^-?\d+$").unwrap());
    static RE_FLOATING: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^-?(?:\d+\.\d*|\.\d+)$").unwrap());
    static RE_STRING: LazyLock<Regex> = LazyLock::new(|| Regex::new("^\".*\"$").unwrap());

    let node: ASTNode = match word {
        // Keywords
        "fun" => ASTNode::Fun,
        "if" => ASTNode::If,
        "then" => ASTNode::Then,
        "else" => ASTNode::Else,
        "in" => ASTNode::In,
        "let" => ASTNode::Let,
        "match" => ASTNode::Match,
        "rec" => ASTNode::Rec,

        // Brackets
        "{" => ASTNode::LBrace,
        "}" => ASTNode::RBrace,
        "(" => ASTNode::LParen,
        ")" => ASTNode::RParen,
        "[" => ASTNode::LSqBra,
        "]" => ASTNode::RSqBra,

        // Reserved Ops
        "->" => ASTNode::Arrow,
        "=>" => ASTNode::DoubleArrow,
        "|" => ASTNode::Bar,
        ":" => ASTNode::Colon,
        "," => ASTNode::Comma,
        ";;" => ASTNode::EndOfStatement,
        "=" => ASTNode::Equals,
        ".." => ASTNode::Spread,

        // Literals
        "{}" => ASTNode::EmptyDict,
        "[]" => ASTNode::EmptyList,
        "()" => ASTNode::Unit,
        _ if { RE_ATOM.is_match(word) } => ASTNode::Atom(word.to_owned()),
        _ if { RE_FLOATING.is_match(word) } => ASTNode::Floating(word.parse::<f64>().unwrap()),
        _ if { RE_INTEGER.is_match(word) } => ASTNode::Integer(word.parse::<i64>().unwrap()),
        _ if { RE_STRING.is_match(word) } => ASTNode::String(
            word.strip_prefix("\"")
                .unwrap()
                .strip_suffix("\"")
                .unwrap()
                .to_owned(),
        ),

        // Identifiers
        _ if { RE_TYPEVAR.is_match(word) } => ASTNode::TypeVar(word.to_owned()),
        _ if { RE_OPERATOR.is_match(word) } => ASTNode::Operator(word.to_owned()),
        _ if { RE_IDENTIFIER.is_match(word) } => ASTNode::Identifier(word.to_owned()),

        // Errors
        _ => ASTNode::Unrecognised(word.to_owned()),
    };

    return match node {
        ASTNode::Unrecognised(w) => Err(format!("Unrecognised token `{w}`")),
        _ => Ok(node),
    };
}

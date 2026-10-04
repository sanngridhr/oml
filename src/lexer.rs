mod token;

use regex::Regex;
use std::sync::LazyLock;
use token::{ASTNode, classify};

// Each part is one kind of token. Order matters: earlier parts win.
const COMMENT: &str = r"(?s:\(\*.*?\*\))"; // (* ... *), may span lines
const STRING: &str = r#""(?:[^"\\\n]|\\[^\n])*""#; // "..." with \" escapes, may be empty
const EMPTY_BRACKETS: &str = r"\(\)|\{\}|\[\]"; // (), {}, []
const BRACKET: &str = r"[(){}\[\],]"; // single brackets and comma
const WORD: &str = r#"[^\s(){}\[\],"]+"#; // anything else up to a separator
const FALLBACK: &str = r"\S"; // stray character, e.g. a lone "

static RE_TOKEN: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = [COMMENT, STRING, EMPTY_BRACKETS, BRACKET, WORD, FALLBACK].join("|");
    Regex::new(&pattern).unwrap()
});

pub(crate) fn lex(code: &str, file: &str) -> Result<Vec<ASTNode>, String> {
    RE_TOKEN
        .find_iter(code)
        .filter(|m: &regex::Match<'_>| !m.as_str().starts_with("(*"))
        .map(|m: regex::Match<'_>| {
            classify(m.as_str()).map_err(|e| {
                let (line, col) = line_col(code, m.start());
                format!("{e} at {file}:{line}:{col}")
            })
        })
        .collect()
}

fn line_col(code: &str, offset: usize) -> (usize, usize) {
    let before = &code[..offset];
    let line = before.matches('\n').count() + 1;
    let col = before.rsplit('\n').next().unwrap().chars().count() + 1;
    (line, col)
}

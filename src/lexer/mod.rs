mod error;
mod lexeme;

use error::{LexingError, LexingErrorKind};
use lexeme::{Lexeme, classify};
use regex::{Match, Regex};
use std::sync::LazyLock;

// Each part is one kind of token. Order matters: earlier parts win.
const COMMENT: &str = r"(?s:\(\*.*?\*\))"; // (* ... *), may span lines
const STRING: &str = r#""(?:[^"\\\n]|\\[^\n])*""#; // "..." with \" escapes, may be empty
const CHAR: &str = r"'.'"; // single character in ''
const EMPTY_BRACKETS: &str = r"\(\)|\{\}|\[\]"; // (), {}, []
const BRACKET: &str = r"[(){}\[\],]"; // single brackets and comma
const WORD: &str = r#"[^\s(){}\[\],"]+"#; // a valid lexeme up to a separator
const FALLBACK: &str = r"\S+"; // anything else 

static RE_TOKEN: LazyLock<Regex> = LazyLock::new(|| {
    let pattern: String = [
        COMMENT,
        STRING,
        CHAR,
        EMPTY_BRACKETS,
        BRACKET,
        WORD,
        FALLBACK,
    ]
    .join("|");
    Regex::new(&pattern).unwrap()
});

pub(crate) fn lex<'src>(source: &'src str) -> Result<Box<[Lexeme<'src>]>, Box<[LexingError]>> {
    let (oks, errs): (Vec<_>, Vec<_>) = RE_TOKEN
        .find_iter(source)
        .filter(|m: &Match<'_>| !m.as_str().starts_with("(*"))
        .map(|m: Match<'_>| {
            classify(m.as_str()).map_err(|kind: LexingErrorKind| LexingError {
                kind,
                span: m.range(),
            })
        })
        .partition(Result::is_ok);

    if errs.is_empty() {
        Ok(oks.into_iter().map(Result::unwrap).collect())
    } else {
        Err(errs.into_iter().map(Result::unwrap_err).collect())
    }
}

pub(crate) mod lexeme;

use lexeme::{Lexeme, classify};
use regex::{Match, Regex};
use std::sync::LazyLock;

// Each part is one kind of token. Order matters: earlier parts win.
const COMMENT: &str = r"(?s:\(\*.*?\*\))"; // (* ... *), may span lines
const STRING: &str = r#""(?:[^"\\\n]|\\[^\n])*""#; // "..." with \" escapes, may be empty
const CHAR: &str = r"'.'"; // single character in ''
const EMPTY_BRACKETS: &str = r"\(\)|\{\}|\[\]"; // (), {}, []
const BRACKET: &str = r"[(){}\[\],]"; // single brackets and comma
const WORD: &str = r#"[^\s(){}\[\],"]+"#; // anything else up to a separator
const FALLBACK: &str = r"\S"; // stray character, e.g. a lone "

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

pub(crate) fn lex<'src>(
    code: &'src str,
    file: Option<&str>,
) -> Result<Vec<Lexeme<'src>>, Box<[String]>> {
    let (oks, errs): (Vec<_>, Vec<_>) = RE_TOKEN
        .find_iter(code)
        .filter(|m| !m.as_str().starts_with("(*"))
        .map(|m| {
            classify(m.as_str()).map_err(|e| {
                let (line, col) = line_col(code, m.start());
                format!("{e} at {}:{line}:{col}", file.unwrap_or_default())
            })
        })
        .partition(Result::is_ok);

    if errs.is_empty() {
        Ok(oks.into_iter().map(Result::unwrap).collect())
    } else {
        Err(errs.into_iter().map(Result::unwrap_err).collect())
    }
}

fn line_col(code: &str, offset: usize) -> (usize, usize) {
    let before: &str = &code[..offset];
    let line: usize = before.matches('\n').count() + 1;
    let col: usize = before.rsplit('\n').next().unwrap().chars().count() + 1;
    (line, col)
}

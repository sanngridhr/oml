use crate::macros::impl_into_u8;
use std::ops::Range;

use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::error::Diagnosable;

#[derive(Debug, Clone, Copy)]
pub enum LexingErrorKind {
    EmptyChar,
    IntegerOverflow,
    UnclosedString,
    Unrecognised,
}
impl_into_u8!(LexingErrorKind);

impl LexingErrorKind {
    fn message(&self) -> &str {
        match &self {
            Self::IntegerOverflow => "integer overflow",
            Self::Unrecognised => "unrecognised lexeme",
            Self::UnclosedString => "unclosed string literal",
            Self::EmptyChar => "empty character literal",
        }
    }

    fn note(&self) -> Option<&str> {
        match &self {
            Self::IntegerOverflow => Some("integer limits are -2,147,483,648..2,147,483,647"),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct LexingError {
    pub(super) kind: LexingErrorKind,
    pub(super) span: Range<usize>,
}

impl Diagnosable for LexingError {
    const CODE_PREFIX: &str = "LX";
    type Kind = LexingErrorKind;

    fn to_diagnostic<FileId>(&self, file_id: FileId) -> Diagnostic<FileId> {
        Diagnostic::error()
            .with_message(self.kind.message())
            .with_label(Label::primary(file_id, self.span.clone()))
            .with_note(self.kind.note().unwrap_or_default())
    }

    fn kind(&self) -> Self::Kind {
        self.kind
    }
}

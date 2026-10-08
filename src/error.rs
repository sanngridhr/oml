use codespan_reporting::diagnostic::Diagnostic;

pub(crate) trait Diagnosable {
    const CODE_PREFIX: &str;
    type Kind: Copy + Into<u8>;

    fn kind(&self) -> Self::Kind;
    fn to_diagnostic<FileId>(&self, file_id: FileId) -> Diagnostic<FileId>;

    fn to_code(&self) -> String {
        format!("{}{:02}", Self::CODE_PREFIX, self.kind().into())
    }
}

pub(crate) trait ToDiagnostics {
    fn to_diagnostics<FileId: Copy>(
        &self,
        file_id: FileId,
    ) -> impl Iterator<Item = Diagnostic<FileId>>;
}

impl<T: Diagnosable> ToDiagnostics for [T] {
    fn to_diagnostics<FileId: Copy>(
        &self,
        file_id: FileId,
    ) -> impl Iterator<Item = Diagnostic<FileId>> {
        self.iter()
            .map(move |e: &T| e.to_diagnostic(file_id).with_code(e.to_code()))
    }
}

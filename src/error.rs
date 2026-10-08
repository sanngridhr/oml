use codespan_reporting::diagnostic::Diagnostic;

pub(crate) trait Diagnosable {
    fn to_diagnostic<FileId>(&self, file_id: FileId) -> Diagnostic<FileId>;
}

pub(crate) struct FileErrors<T: Diagnosable> {
    pub(crate) errors: Vec<T>,
}

impl<T: Diagnosable> FileErrors<T> {
    pub(crate) fn to_diagnostics<FileId: Copy>(
        &self,
        file_id: FileId,
    ) -> impl Iterator<Item = Diagnostic<FileId>> {
        self.errors
            .iter()
            .map(move |e: &T| e.to_diagnostic(file_id))
    }
}

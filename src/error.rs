use std::process::exit;

use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFile,
    term::{
        self, Chars, Config,
        termcolor::{ColorChoice, StandardStream},
    },
};

pub(crate) trait Diagnosable {
    const CODE_PREFIX: &str;
    type Kind: Copy + Into<u8>;

    fn kind(&self) -> Self::Kind;
    fn to_diagnostic<FileId>(&self, file_id: FileId) -> Diagnostic<FileId>;

    fn to_code(&self) -> String {
        format!("{}{:03}", Self::CODE_PREFIX, self.kind().into())
    }
}

pub(crate) trait ToDiagnostics {
    fn report(&self, path: &String, source: &String) -> !;
}

impl<T: Diagnosable> ToDiagnostics for [T] {
    fn report(&self, path: &String, source: &String) -> ! {
        let writer: StandardStream = StandardStream::stderr(ColorChoice::Auto);
        let config: Config = Config {
            chars: Chars::ascii(),
            ..Default::default()
        };
        let files: SimpleFile<String, String> = SimpleFile::new(path.clone(), source.clone());

        self.iter()
            .map(|e: &T| e.to_diagnostic(()).with_code(e.to_code()))
            .for_each(|diagnostic: Diagnostic<()>| {
                term::emit_to_write_style(&mut writer.lock(), &config, &files, &diagnostic).unwrap()
            });

        exit(1);
    }
}

use std::process::exit;

use codespan_reporting::{
    diagnostic::{Diagnostic, Label, Severity},
    files::SimpleFile,
    term::{self, termcolor::ColorChoice, termcolor::StandardStream},
};
use lalrpop_util::{lalrpop_mod, ParseError};

use crate::opcodes::Program;

lalrpop_mod!(pub urcl);

pub fn parse(code: &str, fname: &str) -> (Vec<Program>, (usize, usize, usize, usize)) {
    match urcl::ProgParser::new().parse(code) {
        Ok(ast) => ast,
        Err(e) => {
            let writer = StandardStream::stderr(ColorChoice::Always);
            let config = codespan_reporting::term::Config::default();
            let diag = Diagnostic::new(Severity::Error);
            match e {
                ParseError::ExtraToken { token } => {
                    let diag = diag.with_labels(vec![Label::primary((), token.0..token.2)]);
                    let diag = diag.with_message("Unexpected token.");
                    term::emit(
                        &mut writer.lock(),
                        &config,
                        &SimpleFile::new(fname, code),
                        &diag,
                    )
                    .unwrap();
                    exit(-1)
                }
                ParseError::InvalidToken { location } => {
                    let diag = diag.with_labels(vec![Label::primary((), location..location)]);
                    let diag = diag.with_message("Invalid token.");
                    term::emit(
                        &mut writer.lock(),
                        &config,
                        &SimpleFile::new(fname, code),
                        &diag,
                    )
                    .unwrap();
                    exit(-1)
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    let diag = diag.with_labels(vec![Label::primary((), location..location)]);
                    let diag = diag
                        .with_message(format!("Unexpected end of file. Expected: {:?}", expected));
                    term::emit(
                        &mut writer.lock(),
                        &config,
                        &SimpleFile::new(fname, code),
                        &diag,
                    )
                    .unwrap();
                    exit(-1)
                }
                ParseError::UnrecognizedToken { token, expected } => {
                    let diag = diag.with_labels(vec![Label::primary((), token.0..token.2)]);
                    let diag =
                        diag.with_message(format!("Unexpected token. Expected: {:?}", expected));
                    term::emit(
                        &mut writer.lock(),
                        &config,
                        &SimpleFile::new(fname, code),
                        &diag,
                    )
                    .unwrap();
                    exit(-1)
                }
                _ => exit(-1),
            }
        }
    }
}

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::{traits::Parser, MainParser};

use super::simple_error::ParsingError;

/// A pretty error to report where along parsing the error occurred
///
/// This needs acess to the entire source code in order to display a
/// nice error message, so it should only be used at the 'root parser'
#[derive(Debug, Error, Diagnostic)]
#[error("Error during parsing")]
pub struct PrettyError {
    #[source_code]
    src: String,
    #[label("{}", self.label_message)]
    position: SourceSpan,
    label_message: String,
}

impl<P: Parser> From<(ParsingError, &MainParser<P>)> for PrettyError {
    fn from((perror, pmain): (ParsingError, &MainParser<P>)) -> Self {
        let label_message = match perror.kind {
            super::ParsingErrorKind::MappingError(_)
            | super::ParsingErrorKind::CannotParseAnEmptyString => None,
            super::ParsingErrorKind::PatternNotFound(err) => Some(err),
            super::ParsingErrorKind::CustomError(err) => Some(err),
        }
        .unwrap_or(String::from("Error Here"));
        Self {
            src: pmain.src.clone(),
            // TODO: The length of the error is 0 as of right now
            position: (perror.offset, 0).into(),
            label_message,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    PatternNotFound(String),
    CannotParseAnEmptyString,
    MappingError(String),
}

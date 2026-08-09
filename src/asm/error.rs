#[derive(Debug)]
pub enum AsmError {
    MissingArgument(String),
    InvalidArgument(String),
    UnknownOpcode(String),
    DuplicateLabel(String),
    UnknownLabel(String),
}

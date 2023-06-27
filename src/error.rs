use thiserror::Error;

/// Error that can be generated during the lexing phase of the interpreter.
#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("an unexpected character was found while lexing: {0}")]
    UnexpectedCharacter(char),
}
/// Error that is generated during interpretation.
#[derive(Error, Debug)]
pub enum RuntimeError {}

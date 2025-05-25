pub type CalcResult<T> = Result<T, CalcError>;
#[derive(Debug, PartialEq, thiserror::Error, Clone)]
pub enum CalcError {
    #[error("Unexpected character: {0}")]
    UnexpectedChar(char),
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),
    #[error("Undeclared method: {0}")]
    UndeclaredMethod(String),
    #[error("Illegal method arguments: {0}")]
    IllegalMethodArgument(String)
}

use function::Function;

use crate::error::{CalcError, CalcResult};

pub mod cos;
pub mod function;
pub mod sin;
pub fn get_function_by_name(name: &str) -> CalcResult<Box<dyn Function>> {
    match name {
        "cos" => Ok(Box::new(cos::Cos {})),
        "sin" => Ok(Box::new(sin::Sin {})),
        _ => Err(CalcError::UndeclardMethod("".to_string())),
    }
}

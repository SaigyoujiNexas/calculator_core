use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive}, Decimal, MathematicalOps
};

use crate::error::{CalcError, CalcResult};

use super::function::Function;

#[derive(Debug)]
pub struct Cos {}

impl Function for Cos {
    fn get_method_name(&self) -> String {
        "cos".to_string()
    }

    fn eval(&self, params: &Vec<crate::ast::Node>) -> CalcResult<Decimal> {
        if params.len() != 1 {
            return Err(CalcError::IllegalMethodArgument(format!(
                "Illegal number of arguments, expected 1, got {}",
                params.len()
            )));
        }
        Ok(params.first().unwrap().eval()?.cos())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn equals(&self, other: &dyn Function) -> bool {
        other.as_any().downcast_ref::<Cos>().is_some()
    }
}

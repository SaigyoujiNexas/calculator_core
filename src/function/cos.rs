use rust_decimal::{
    Decimal,
    prelude::{FromPrimitive, ToPrimitive},
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
        let param = params.first().unwrap();
        let val = param
            .eval()
            ?.to_f64()
            .ok_or_else(|| {
                CalcError::IllegalMethodArgument("Illegal number of arguments, expected float".to_string())
            })?
            .cos();

        Ok(Decimal::from_f64(val).unwrap())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn equals(&self, other: &dyn Function) -> bool {
        other.as_any().downcast_ref::<Cos>().is_some()
    }
}

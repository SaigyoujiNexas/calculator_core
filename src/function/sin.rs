use rust_decimal::{
    Decimal,
    prelude::{FromPrimitive, ToPrimitive},
};

use crate::{
    ast::Node,
    error::{CalcError, CalcResult},
};

use super::function::Function;

#[derive(Debug)]
pub struct Sin {}
impl Function for Sin {
    fn get_method_name(&self) -> String {
        "sin".to_string()
    }

    fn eval(&self, params: &Vec<Node>) -> CalcResult<Decimal> {
        if params.len() != 1 {
            return Err(CalcError::IllegalMethodArgument(format!(
                "Illegal number of arguments, expected 1, got {}",
                params.len()
            )));
        }
        let param =
            params.first().unwrap().eval()?.to_f64().ok_or_else(|| {
                CalcError::IllegalMethodArgument("Illegal number type".to_string())
            })?;
        Ok(Decimal::from_f64(param.sin()).unwrap())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn equals(&self, other: &dyn Function) -> bool {
        other.as_any().downcast_ref::<Sin>().is_some()
    }
}

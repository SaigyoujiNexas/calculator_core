use std::{any::{Any}, fmt::Debug};

use rust_decimal::Decimal;

use crate::{ast::Node, error::CalcResult};

pub trait Function: Debug + Any {
    fn get_method_name(&self) -> String;
    fn eval(&self, params: &Vec<Node>) -> CalcResult<Decimal>;
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, other: &dyn Function) -> bool;
}

impl PartialEq for dyn Function
{
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}


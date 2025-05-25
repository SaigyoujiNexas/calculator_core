use std::fmt::Debug;

use rust_decimal::{Decimal, MathematicalOps};

use crate::{error::CalcResult, function::function::Function};


#[derive(Debug, PartialEq)]
pub enum Node{
    Add(Box<Node>,Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    FunctionCall(Box<dyn Function>, Vec<Node>),
    Number(Decimal)
}
impl Node {
    pub fn eval(&self) -> CalcResult<Decimal> {
        use Node::*;
        match self {
            Add(left, right) => Ok(left.eval()? + &right.eval()?),
            Sub(left, right) => Ok(left.eval()? - &right.eval()?),
            Mul(left, right) => Ok(left.eval()? * &right.eval()?),
            Div(left, right) => Ok(left.eval()? / &right.eval()?),
            Pow(left, right) => Ok(left.eval()?.powd(right.eval()?)),
            Negative(expr) => Ok(-expr.eval()?),
            FunctionCall(f, params) => f.eval(params),
            Number(num) => Ok(num.clone()),
        }
    }
}



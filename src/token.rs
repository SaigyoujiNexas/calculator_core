use std::fmt::Display;

use rust_decimal::Decimal;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Caret,
    Number(Decimal),
    FunctionIdentifier(String),
    FunctionParamSpliter,
    LeftParen,
    RightParen,
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Caret => write!(f, "^"),
            Number(x) => write!(f, "{}", x),
            FunctionIdentifier(x) => write!(f, "{}", x),
            FunctionParamSpliter => write!(f, ", "),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            EOF => write!(f, ""),
            Div => write!(f, "/"),
        }
    }
}

impl Token {
    pub fn get_precedence(&self) -> OperatorPrecedence {
        println!("get precedence: {}", self);
        use OperatorPrecedence::*;
        use Token::*;
        match self {
            Add | Sub => AddOrSub,
            Mul | Div => MulOrDiv,
            Caret => Pow,
            FunctionIdentifier(_) => Function,
            _ => Default,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum OperatorPrecedence {
    Default,
    AddOrSub,
    MulOrDiv,
    Pow,
    Negative,
    Function,
}

impl Display for OperatorPrecedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OperatorPrecedence::*;
        match self {
            Default => write!(f, "Default"),
            AddOrSub => write!(f, "AddOrSub"),
            MulOrDiv => write!(f, "MulOrDiv"),
            Pow => write!(f, "Pow"),
            Negative => write!(f, "Negative"),
            Function => write!(f, "Function"),
        }
    }
}
use error::CalcResult;
use parser::Parser;
use rust_decimal::Decimal;

mod token;
mod tokenizer;
mod ast;
mod error;
mod parser;
mod function;

pub fn calculate(expression: &str) -> CalcResult<Decimal> {
    Parser::new(expression)?.parse()?.eval()
}


#[cfg(test)]
mod tests {
}

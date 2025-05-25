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
    use rust_decimal::Decimal;

    use crate::calculate;

    #[test]
    fn test_calc() {
        // 27 + 3 * 5 = 27 + 15 = 42
        let expression = "(cos(0) * 3) ^ 3 + 3 * ( 7 - 2)";
        let ans = calculate(expression).unwrap();
        assert_eq!(ans, Decimal::from(42))
        
    }
}

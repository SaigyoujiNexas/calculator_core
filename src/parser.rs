use crate::{
    ast::Node,
    error::{CalcError, CalcResult},
    function::get_function_by_name,
    token::{OperatorPrecedence, Token},
    tokenizer::Tokenizer,
};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expression: &'a str) -> CalcResult<Self> {
        let mut tokenizer = Tokenizer::new(expression);
        let current_token = tokenizer
            .next()
            .ok_or_else(|| CalcError::UnexpectedChar(tokenizer.get_unexpected_char().unwrap()))?;
        Ok(Self {
            tokenizer,
            current_token,
        })
    }
    pub fn parse(&mut self) -> CalcResult<Node> {
        return self.parse_expression(OperatorPrecedence::Default);
    }
}

impl Parser<'_> {
    fn next_token(&mut self) -> CalcResult<()> {
        self.current_token = self.tokenizer.next().ok_or_else(|| {
            CalcError::UnexpectedChar(self.tokenizer.get_unexpected_char().unwrap())
        })?;
        Ok(())
    }

    fn parse_expression(&mut self, precedence: OperatorPrecedence) -> CalcResult<Node> {
        let mut expr = self.parse_number()?;
        println!("current precedence: {}", self.current_token.get_precedence());
        while precedence < self.current_token.get_precedence()
        {
            expr = self.parse_binary_expression(expr)?;
        }
        Ok(expr)
    }
    fn parse_number(&mut self) -> CalcResult<Node> {
        match self.current_token {
            Token::Number(number) => {
                self.next_token()?;
                Ok(Node::Number(number))
            }
            Token::Sub => {
                self.next_token()?;
                let expr = self.parse_expression(OperatorPrecedence::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::LeftParen => {
                self.next_token()?;
                let expr = self.parse_expression(OperatorPrecedence::Default)?;
                if self.current_token != Token::RightParen {
                    if self.current_token == Token::EOF {
                        return Err(CalcError::InvalidOperator(
                            "Missing right paren".to_string(),
                        ));
                    }
                    return Err(CalcError::InvalidOperator(format!(
                        "Expected right paren, got {}",
                        self.current_token
                    )));
                }
                self.next_token()?;
                Ok(expr)
            }
            Token::FunctionIdentifier(ref identifier) => {
                let function = get_function_by_name(&identifier.clone())?;
                println!("func ident: {}", self.current_token);
                self.next_token()?;
                if self.current_token != Token::LeftParen {
                    return Err(CalcError::InvalidOperator(format!(
                        "Expected '(', got '{}'",
                        self.current_token
                    )));
                }
                self.next_token()?;
                let mut params: Vec<Node> = vec![];
                while self.current_token != Token::RightParen {
                    println!("start parse: {}", self.current_token);
                    let param = self.parse_expression(OperatorPrecedence::Default)?;
                    params.push(param);

                    println!("end parse");
                    if self.current_token != Token::FunctionParamSpliter
                        && self.current_token != Token::RightParen
                    {
                        return Err(CalcError::InvalidOperator(format!(
                            "Expected ',' or ')', got '{}'",
                            self.current_token
                        )));
                    }
                    if self.current_token == Token::FunctionParamSpliter {
                        self.next_token()?;
                    }
                }
                if self.current_token != Token::RightParen {
                    return Err(CalcError::InvalidOperator(format!(
                        "Expected ')', got '{}'",
                        self.current_token
                    )));
                }
                self.next_token()?;
                println!("passed function call, current token: {}", self.current_token);
                Ok(Node::FunctionCall(function, params))
            }
            _ => {
                if self.current_token == Token::EOF {
                    println!("test, {}", self.current_token);
                    return Err(CalcError::InvalidOperator(
                        "Not a around expression".to_string(),
                    ));
                }
                Err(CalcError::InvalidOperator(format!(
                    "Expected number or expression, got {}",
                    self.current_token
                )))
            }
        }
    }
    fn parse_binary_expression(&mut self, left_expr: Node) -> CalcResult<Node> {
        match self.current_token {
            Token::Add => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::AddOrSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Sub => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::AddOrSub)?;
                Ok(Node::Sub(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Mul => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::MulOrDiv)?;
                Ok(Node::Mul(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Div => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::MulOrDiv)?;
                Ok(Node::Div(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::Pow)?;
                Ok(Node::Pow(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => {
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::{Decimal, prelude::FromPrimitive};

    use crate::function::{cos, sin};

    use super::*;

    #[test]
    fn test_parse_number() {
        let mut parser0 = Parser::new("114.514").unwrap();
        let parsed = parser0.parse_number().unwrap();
        assert_eq!(parsed, Node::Number(Decimal::from_f64(114.514).unwrap()));
        let mut parser1 = Parser::new("cos(1, 3) ^ 4").unwrap();
        let parsed = parser1.parse_expression(OperatorPrecedence::Default).unwrap();
        assert_eq!(
            parsed,
            Node::Pow(Box::new(Node::FunctionCall(
                Box::new(cos::Cos {}),
                vec![
                    Node::Number(Decimal::from(1)),
                    Node::Number(Decimal::from(3))
                ]
            )), Box::new(Node::Number(Decimal::from(4))))
        );
        println!("111");
        let mut parser3 = Parser::new("1 + 2 * sin(7) ^ 5 * (6 - 7 ^ 2)").unwrap();
        let parsed = parser3
            .parse_expression(OperatorPrecedence::Default)
            .unwrap();
        assert_eq!(
            parsed,
            Node::Add(
                Box::new(Node::Number(Decimal::from(1))),
                Box::new(Node::Mul(
                    Box::new(Node::Mul(
                        Box::new(Node::Number(Decimal::from(2))),
                        Box::new(Node::Pow(
                            Box::new(Node::FunctionCall(
                                Box::new(sin::Sin {}),
                                vec![Node::Number(Decimal::from(7))]
                            )),
                            Box::new(Node::Number(Decimal::from(5)))
                        ))
                    )),
                    Box::new(Node::Sub(
                        Box::new(Node::Number(Decimal::from(6))),
                        Box::new(Node::Pow(
                            Box::new(Node::Number(Decimal::from(7))),
                            Box::new(Node::Number(Decimal::from(2)))
                        ))
                    ))
                ))
            )
        )
    }
}

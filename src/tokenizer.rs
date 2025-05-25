use std::{iter::Peekable, str::Chars};

use crate::token::Token;

pub struct Tokenizer<'a> {
    expression: Peekable<Chars<'a>>,
    is_end: bool,
    unexpected_char: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expression: &'a str) -> Self {
        Self {
            expression: expression.chars().peekable(),
            is_end: false,
            unexpected_char: None,
        }
    }

    pub fn get_unexpected_char(&self) -> Option<char> {
        return self.unexpected_char;
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end {
            return None;
        }
        let next_char = self.expression.next();
        match next_char {
            Some(chr) if chr.is_numeric() => {
                let mut number = String::from(chr);
                let mut meet_dot = false;
                while let Some(next_num_chr) = self
                    .expression
                    .next_if(|c| c.is_numeric() || (*c == '.' && !meet_dot))
                {
                    if next_num_chr == '.' {
                        meet_dot = true;
                    }
                    number.push(next_num_chr);
                }
                println!("lixinyan, number: {}", number);
                Some(Token::Number(number.parse().unwrap()))
            }
            Some(chr) if chr.is_whitespace() => {
                while self.expression.next_if(|c| c.is_whitespace()).is_some() {}
                self.next()
            }
            Some(',') => Some(Token::FunctionParamSpliter),
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Sub),
            Some('*') => Some(Token::Mul),
            Some('/') => Some(Token::Div),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some(chr) if chr.is_alphabetic() || chr == '_' => {
                let mut identifier = String::from(chr);
                while let Some(next_indentifier) = self
                    .expression
                    .next_if(|c| c.is_alphanumeric() || *c == '_')
                {
                    identifier.push(next_indentifier);
                }
                Some(Token::FunctionIdentifier(identifier.to_string()))
            }
            Some(chr) => {
                self.is_end = true;
                self.unexpected_char = Some(chr);
                None
            }
            None => {
                self.is_end = true;
                Some(Token::EOF)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::token::Token;

    use super::Tokenizer;

    #[test]
    fn test_tokenizer() {
        let token1 = Tokenizer::new("1 + 2 * 3 - 4 ^ 5").collect::<Vec<Token>>();
        assert_eq!(
            token1,
            vec![
                Token::Number(1.into()),
                Token::Add,
                Token::Number(2.into()),
                Token::Mul,
                Token::Number(3.into()),
                Token::Sub,
                Token::Number(4.into()),
                Token::Caret,
                Token::Number(5.into()),
                Token::EOF
            ]
        );
        let token2 = Tokenizer::new("2 * cos(3)").collect::<Vec<Token>>();
        assert_eq!(
            token2,
            vec![
                Token::Number(2.into()),
                Token::Mul,
                Token::FunctionIdentifier("cos".to_string()),
                Token::LeftParen,
                Token::Number(3.into()),
                Token::RightParen,
                Token::EOF
            ]
        );
        let token3 = Tokenizer::new("3 * test(2 + 3, 6 * 7) + 2").collect::<Vec<Token>>();
        assert_eq!(
            token3,
            vec![
                Token::Number(3.into()),
                Token::Mul,
                Token::FunctionIdentifier("test".to_string()),
                Token::LeftParen,
                Token::Number(2.into()),
                Token::Add,
                Token::Number(3.into()),
                Token::FunctionParamSpliter,
                Token::Number(6.into()),
                Token::Mul,
                Token::Number(7.into()),
                Token::RightParen,
                Token::Add,
                Token::Number(2.into()),
                Token::EOF
            ]
        );
    }
}

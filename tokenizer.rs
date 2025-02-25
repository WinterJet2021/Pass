use std::iter::Peekable;
use std::str::Chars;
use super::token::Token;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }

    fn parse_number(&mut self, first_digit: char) -> Option<Token> {
        let mut num_str = first_digit.to_string();

        while let Some(&next) = self.expr.peek() {
            if next.is_ascii_digit() || next == '.' {
                num_str.push(self.expr.next().unwrap());
            } else {
                break;
            }
        }

        match num_str.parse::<f64>() {
            Ok(value) => Some(Token::Num(value)),
            Err(_) => None,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        while let Some(&c) = self.expr.peek() {
            match c {
                '0'..='9' => {
                    self.expr.next();
                    return self.parse_number(c);
                }
                '+' => {
                    self.expr.next();
                    return Some(Token::Add);
                }
                '-' => {
                    self.expr.next();
                    return Some(Token::Subtract);
                }
                '*' => {
                    self.expr.next();
                    return Some(Token::Multiply);
                }
                '/' => {
                    self.expr.next();
                    return Some(Token::Divide);
                }
                '^' => {
                    self.expr.next();
                    return Some(Token::Caret);
                }
                '&' => {
                    self.expr.next();
                    return Some(Token::And);
                }
                '|' => {
                    self.expr.next();
                    return Some(Token::Or);
                }
                '(' => {
                    self.expr.next();
                    return Some(Token::LeftParen);
                }
                ')' => {
                    self.expr.next();
                    return Some(Token::RightParen);
                }
                ' ' | '\t' | '\n' => {
                    self.expr.next();
                }
                _ => {
                    self.expr.next();
                    return None;
                }
            }
        }
        Some(Token::EOF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_decimal_number() {
        let mut tokenizer = Tokenizer::new("3.14");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(3.14));
    }

    #[test]
    fn test_tokenize_mixed_operators() {
        let mut tokenizer = Tokenizer::new("2+3*4-5/2");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(2.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Add);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(3.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Multiply);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(4.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Subtract);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(5.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Divide);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(2.0));
    }

    #[test]
    fn test_tokenize_unrecognized_character() {
        let mut tokenizer = Tokenizer::new("2+3$4");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(2.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Add);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(3.0));
        assert!(tokenizer.next().is_none());
    }

    #[test]
    fn test_tokenize_empty_string() {
        let mut tokenizer = Tokenizer::new("");
        assert_eq!(tokenizer.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_tokenize_whitespace() {
        let mut tokenizer = Tokenizer::new("   4   +  6 ");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(4.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Add);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(6.0));
    }
}

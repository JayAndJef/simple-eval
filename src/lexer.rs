use crate::parser::Operator;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Literal(f64),
    Plus,
    Minus,
    Div,
    Mul,
    Exp,
    LParen,
    RParen,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError;

pub struct Lexer {
    remaining: String,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            remaining: source.to_string(),
        }
    }

    pub fn next_token(&mut self) -> Result<Option<TokenKind>, ParseError> {
        if self.remaining.len() == 0 {
            return Ok(None);
        }

        self.remaining = self.remaining.trim().to_string();

        let mut eaten_chars = 1usize;
        let result = match self.remaining.chars().next().unwrap() {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Mul,
            '/' => TokenKind::Div,
            '^' => TokenKind::Exp,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '0'..='9' => {
                let num_str = self
                    .remaining
                    .chars()
                    .take_while(|c| *c == '.' || c.is_digit(10))
                    .collect::<String>();
                eaten_chars = num_str.chars().count();

                TokenKind::Literal(num_str.parse::<f64>().unwrap())
            }
            _ => return Err(ParseError),
        };

        self.remaining = self.remaining[eaten_chars..].to_string();

        Ok(Some(result))
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::TokenKind::*;

    #[test]
    fn always_passes() {
        assert_eq!(2, 1 + 1);
    }

    #[test]
    #[should_panic]
    fn test_err() {
        let mut lx = Lexer::new("asbd");
        let mut token_vec = Vec::new();
        while let Some(token) = lx.next_token().unwrap() {
            token_vec.push(token);
        }
    }

    #[test]
    fn test_operators() {
        let mut lx = Lexer::new("+-/*^()");
        let mut token_vec = Vec::new();
        while let Some(token) = lx.next_token().unwrap() {
            token_vec.push(token);
        }

        assert_eq!(token_vec, [Plus, Minus, Div, Mul, Exp, LParen, RParen]);
    }

    #[test]
    fn test_number() {
        let mut lx = Lexer::new("103.56");
        let mut token_vec = Vec::new();
        while let Some(token) = lx.next_token().unwrap() {
            token_vec.push(token);
        }

        assert_eq!(token_vec, [Literal(103.56)])
    }

    #[test]
    fn test_all() {
        let mut lx = Lexer::new("56.5 + (67 - 8)^5");
        let mut token_vec = Vec::new();
        while let Some(token) = lx.next_token().unwrap() {
            token_vec.push(token);
        }

        assert_eq!(
            token_vec,
            [
                Literal(56.5),
                Plus,
                LParen,
                Literal(67.0),
                Minus,
                Literal(8.0),
                RParen,
                Exp,
                Literal(5.0)
            ]
        )
    }
}

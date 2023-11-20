mod token;
use std::ops::Range;

pub use token::{Token, TokenType};


pub struct Lexer { 
    pub chars: Vec<char>,
    pos: usize,
}

impl Lexer { 
    pub fn new(code: &str) -> Self {
        Self {
            chars: Vec::from_iter(code.chars()),
            pos: 0
        }
    }

    fn builder(&mut self, init_value: &char, range: Range<char>) -> String {
        let mut value = String::from(*init_value);

        while let Some(ch) = self.chars.get(self.pos + 1) {
            if !range.contains(ch) { break }
            value.push(*ch);
            self.pos += 1;
        }

        value
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let ch: char = *self.chars.get(self.pos)?;

        let (_type, value) = match ch {
            '0'..='9' => (
                TokenType::Number, 
                self.builder(&ch, Range { start: '0', end: '9' })
            ),

            '*' | '/' | '+' | '-' => (
                TokenType::Operator, 
                String::from(ch)
            ),

            '(' => (
                TokenType::LParen,
                String::from(ch)
            ),
            ')' => (
                TokenType::RParen, 
                String::from(ch)
            ),

            ' ' => {
                self.pos += 1;
                return self.next_token()
            },
            _ => panic!("Unable to parse token {} at {}!", ch, self.pos)
        };

        self.pos += 1;

        Some(Token { _type, value })
    }
}

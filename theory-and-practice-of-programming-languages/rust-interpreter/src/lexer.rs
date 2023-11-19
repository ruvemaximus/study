mod token;
use core::fmt;
use std::{ops::Range, str::FromStr};

pub use token::Token;


// todo Make impl Iter for Lexer
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

    fn build_token<T>(&mut self, init_value: &char, range: Range<char>) -> T 
        where T: FromStr, T::Err: fmt::Debug 
    {
        let mut value = String::from(*init_value);

        while let Some(ch) = self.chars.get(self.pos + 1) {
            if !range.contains(ch) { break }
            value.push(*ch);
            self.pos += 1;
        }

        value.parse::<T>().unwrap()
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let ch: char = *self.chars.get(self.pos)?;

        let token = match ch {
            '0'..='9' => {
                Token::Number(
                    self.build_token(&ch, Range { start: '0', end: '9' })
                )
            },

            '*' | '/' | '+' | '-' => Token::Operator(ch),

            '(' => Token::OpenParen,
            ')' => Token::CloseParen,

            ' ' => {
                self.pos += 1;
                return self.next_token()
            },
            _ => panic!("Unable to parse token at {}!", self.pos)
        };

        self.pos += 1;

        Some(token)
    }
}

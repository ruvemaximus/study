#[derive(Debug)]
pub enum Token {
    Int(i32),
    Mul,
    Div,
    Plus,
    Minus,
    OpenParen,
    CloseParen,
}

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

    pub fn next_token(&mut self) -> Option<Token> {
        let ch: char = *self.chars.get(self.pos)?;

        let token = match ch {
            '0'..='9' => {
                let mut number = String::from(ch);

                while let Some(ch) = self.chars.get(self.pos + 1) {
                    if !('0'..='9').contains(ch) { break }
                    number.push(*ch);
                    self.pos += 1;
                }

                Token::Int(number.parse().unwrap())
            },
            '*' => Token::Mul,
            '-' => Token::Minus,
            '+' => Token::Plus,
            '/' => Token::Div,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            ' ' => {
                self.pos += 1;
                return self.next_token()
            },
            _ => panic!("Unable to parse token!")
        };

        self.pos += 1;

        Some(token)
    }
}

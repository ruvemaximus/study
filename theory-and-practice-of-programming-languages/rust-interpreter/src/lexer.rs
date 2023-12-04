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

    fn build_until_space(&mut self, init_value: &char, range: Range<char>) -> String {
        let mut value = String::from(*init_value);

        while let Some(ch) = self.chars.get(self.pos + 1) {
            if !range.contains(ch) { break }
            value.push(*ch);
            self.pos += 1;
        }

        value
    }

    fn expect_chars(&mut self, init_value: &char, expected: String) -> String {
        let mut value = String::from(*init_value);

        for expected_char in expected.chars() {
            self.pos += 1;
            let ch = *self.chars.get(self.pos).unwrap();

            if ch == ' ' {
                continue;
            }

            if !(ch == expected_char) {
                panic!("[lexer] Ожидался символ '{expected_char}' на позиции {}, получен '{ch}'!", self.pos)
            }

            value.push(ch);
        }
        
        value
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let ch: char = *self.chars.get(self.pos)?;

        let (_type, value) = match ch {
            '0'..='9' => (
                TokenType::Number, 
                self.build_until_space(&ch, Range { start: '0', end: '9' })
            ),
            'a'..='z' => (
                TokenType::ID,
                self.build_until_space(&ch, Range { start: 'a', end: 'z' })
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
            ';' => (
                TokenType::SEMI,
                String::from(ch)
            ),
            ':' => (
                TokenType::Assign, 
                self.expect_chars(&ch, String::from("="))
            ),
            'B' => (
                TokenType::Begin, 
                self.expect_chars(&ch, String::from("EGIN"))
            ),
            'E' => (
                TokenType::End, 
                self.expect_chars(&ch, String::from("ND."))
            ),
            ' ' => {
                self.pos += 1;
                return self.next_token()
            },
            _ => panic!("[lexer] Неизвестный символ '{ch}' на позиции {}!", self.pos)
        };

        self.pos += 1;

        Some(Token { _type, value })
    }
}


#[test]
fn tokenize_digit() {
    let mut lexer = Lexer::new(&"2");

    assert_eq!(
        lexer.next_token(), 
        Some(Token { _type: TokenType::Number, value: "2".to_string()} )
    );
}

#[test]
fn tokenize_assign() {
    let mut lexer = Lexer::new(&":=");

    assert_eq!(
        lexer.next_token(), 
        Some(Token { _type: TokenType::Assign, value: ":=".to_string()} )
    );
}

#[test]
fn tokenize_begin() {
    let mut lexer = Lexer::new(&"BEGIN");

    assert_eq!(
        lexer.next_token(),
        Some(Token { _type: TokenType::Begin, value: "BEGIN".to_string()} )
    );
}

#[test]
fn tokenize_end() {
    let mut lexer = Lexer::new(&"END.");
    assert_eq!(
        lexer.next_token(),
        Some(Token { _type: TokenType::End, value: "END.".to_string()} )
    );
}

#[test]
fn tokenize_id() {
    let mut lexer = Lexer::new(&"x");
    assert_eq!(
        lexer.next_token(),
        Some(Token { _type: TokenType::ID, value: "x".to_string()} )
    );
}

#[test]
#[should_panic(expected="[lexer] Ожидался символ '=' на позиции 1, получен ':'!")]
fn unexpected_char() {
    let mut lexer = Lexer::new(&"::");
    lexer.next_token();
}

#[test]
fn tokenize_number() {
    let mut lexer = Lexer::new(&"123");

    assert_eq!(
        lexer.next_token(), 
        Some(Token { _type: TokenType::Number, value: "123".to_string()} )
    );
}

#[test]
fn tokenize_lparen() {
    let mut lexer = Lexer::new(&"(");

    assert_eq!(
        lexer.next_token(),
        Some(Token { _type: TokenType::LParen, value: "(".to_string() })
    );
}

#[test]
fn tokenize_rparen() {
    let mut lexer = Lexer::new(&")");

    assert_eq!(
        lexer.next_token(),
        Some(Token { _type: TokenType::RParen, value: ")".to_string() })
    );
}

#[test]
#[should_panic(expected="[lexer] Неизвестный символ '@' на позиции 0!")]
fn unexpected_token() {
    let mut lexer = Lexer::new(&"@");
    lexer.next_token();
}

#[test]
fn tokenize_operators() {
    let code = &"*/+-";
    let mut lexer = Lexer::new(code);

    for ch in code.chars() {
        assert_eq!(
            lexer.next_token(),
            Some(Token{ _type: TokenType::Operator, value: ch.to_string() })
        );
    }
}

#[test]
fn ignore_spaces() {
    let code = &"  1  ";
    let mut lexer = Lexer::new(code);
    
    while let Some(token) = lexer.next_token() {
        assert_eq!(token, Token { _type: TokenType::Number, value: "1".to_string() })
    }
}

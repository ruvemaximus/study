mod lexer {
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
        pub fn new(chars: Vec<char>) -> Self {
            Self {
                chars,
                pos: 0
            }
        }

        pub fn next_token(&mut self) -> Option<Token> {
            let mut ch: char = *self.chars.get(self.pos)?;

            let token = match ch {
                '0'..='9' => {
                    let mut number = ch.to_digit(10)? as i32;

                    loop {
                        ch = match self.chars.get(self.pos + 1) {
                            Some(ch) => *ch,
                            None => break Token::Int(number)
                        };

                        match ch {
                            '0'..='9' => {
                                let digit = ch.to_digit(10)? as i32;
                                number = number * 10 + digit;
                                self.pos += 1;
                            },
                            _ => break Token::Int(number)
                        }
                    }
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
}


pub fn parse(code: &str) {
    let mut lexer = lexer::Lexer::new(Vec::from_iter(code.chars()));

    dbg!(lexer.next_token());
    dbg!(lexer.next_token());
    dbg!(lexer.next_token());
}

mod lexer {
    use std::str::Chars;


    pub enum Token {
        Int(i8),
        Mul,
        Div,
        Plus,
        Minus,
        OpenParen,
        CloseParen,
    }

    fn number(chars: &mut Chars, value: u32) -> Option<Token> {
        let mut number = Token::Int(value as i8);

        while let Some(digit) = chars.next()?.to_digit(10) {
            
        } 

        Token::Int(10);

        Some(number)
    }

    pub fn next_token(chars: &mut Chars) -> Token {
        let c = chars.next().unwrap();
        match c {
            '0'..='9' => number(chars, c.to_digit(10).unwrap()).unwrap(),
            '*' => Token::Mul,
            '-' => Token::Minus,
            '+' => Token::Plus,
            '/' => Token::Div,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
             _ => panic!("Unable to parse token!")
        }
    }
}


pub fn parse(code: &str) {
    let mut chars = code.chars();
    lexer::next_token(&mut chars);
}

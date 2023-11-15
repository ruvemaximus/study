#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Dev
}


#[derive(Debug, PartialEq)]
pub enum Bracket {
    Open,
    Close
}


#[derive(Debug, PartialEq)]
pub enum Token {
    Number(u32),
    Operator(Operator),
    Bracket(Bracket)
}


// todo Написать структуру LexerError
pub fn lexer(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for (i, c) in code.chars().enumerate() {
        let current_token = match c {
            '0'..='9' => {
                let digit = c.to_digit(10).unwrap();

                match tokens.pop() {
                    Some(Token::Number(value)) => {
                        Token::Number(digit + value * 10)
                    },
                    Some(last_token) => {
                        tokens.push(last_token);
                        Token::Number(digit)
                    },
                    None => Token::Number(digit)
                }
            },
            '*' => Token::Operator(Operator::Mul),
            '/' => Token::Operator(Operator::Dev),
            '+' => Token::Operator(Operator::Plus),
            '-' => Token::Operator(Operator::Minus),
            '(' => Token::Bracket(Bracket::Open),
            ')' => Token::Bracket(Bracket::Close),
            _ => panic!("Wrong character '{c}' at position {i}!")
        };

        tokens.push(current_token);
    }

    tokens
}


#[test]
fn test_lexer_12_plus_2() {
    let tokens = lexer(&"12+2".to_string());

    assert_eq!(
        Vec::from([
            Token::Number(12), 
            Token::Operator(Operator::Plus), 
            Token::Number(2)
        ]),
        tokens
    )
}

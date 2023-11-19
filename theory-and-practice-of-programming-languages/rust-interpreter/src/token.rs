#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Operator(char),
    OpenParen,
    CloseParen,
}


impl Token {
    pub fn compare_types(token: &Token, expected: &Token) -> bool {
        let panic = || panic!("Check token failed!");

        // ? Maybe write macros?
        match (token, expected) {
            (Token::Operator(_), Token::Operator(_)) => {},
            (_, Token::Operator(_)) => panic(),

            (Token::Number(_), Token::Number(_)) => {}, 
            (_, Token::Number(_)) => panic(),

            (l @ _, r @ _) if l != r => panic(),
            (_, _) => {}
        }

        true
    }
}


#[test]
fn compare_operator_types_success() {
    let token = Token::Operator('-');
    let expected = Token::Operator('+');

    assert!(Token::compare_types(&token, &expected));
}

#[test]
#[should_panic(expected="Check token failed!")]
fn compare_operator_types_fail() {
    let token = Token::Operator('-');
    let expected = Token::OpenParen;

    Token::compare_types(&token, &expected);
}

#[test]
fn compare_number_types_success() {
    let token = Token::Number(0);
    let expected = Token::Number(1);

    assert!(Token::compare_types(&token, &expected));
}


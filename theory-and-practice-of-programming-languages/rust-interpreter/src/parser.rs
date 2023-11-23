#[path = "lexer.rs"]
mod lexer;

use lexer::{Token, TokenType, Lexer};


#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Number(i32),
    BinOp { left: Box<Self>, op: String, right: Box<Self> },
    UnaryOp { op: String, value: Box<Self> }
}

pub struct Parser {
    lexer: lexer::Lexer,
    current_token: Option<Token>
}


impl Parser {
    pub fn new(code: &str) -> Self {
        let mut lexer = Lexer::new(&code);
        let current_token = lexer.next_token();

        Self { current_token, lexer }
    }

    fn check_token(&mut self, expected: TokenType) {
        if let Some(token) = &self.current_token {
            if token._type != expected {
                panic!("[grammar] Ожидался токен c типом {:?}, получен тип {:?}", expected, token._type)
            }
            self.step()
        }
    }

    fn step(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn fact(&mut self) -> Option<ASTNode> {
        if let Some(Token { _type: t, value: v }) = &self.current_token.clone() {
            let node = match t {
                TokenType::Number => {
                    self.step();
                    ASTNode::Number(v.parse().unwrap())
                },
                TokenType::LParen => {
                    self.check_token(TokenType::LParen);
                    let expression = self.expr().unwrap();
                    self.check_token(TokenType::RParen);
                    expression
                },
                TokenType::Operator => {
                    if !['-', '+'].map(|c| String::from(c)).contains(&v) {
                        panic!("[grammar] Неожиданный оператор '{v}'!");
                    }
                    self.check_token(TokenType::Operator);
                    self.check_token(TokenType::LParen);
                    let expression = self.expr().unwrap();
                    self.check_token(TokenType::RParen);
                    ASTNode::UnaryOp { 
                        op: v.to_string(), 
                        value: Box::new(expression) 
                    }
                }
                _ => panic!("[grammar] Неожиданный токен {t:?} '{v}'!")
            };

            return Some(node);
        }
        None
    }

    fn term(&mut self) -> Option<ASTNode> {
        let mut result = self.fact()?;

        while let Some(token) = self.current_token.clone() {
            if !['*', '/'].map(|c| String::from(c)).contains(&token.value) {
                break;
            }

            if token._type != TokenType::Operator {
                break;
            }

            self.step();

            result = ASTNode::BinOp { 
                left: Box::new(result), 
                op: token.value, 
                right: Box::new(self.term().unwrap())
            };
        }
        Some(result)
    }

    fn expr(&mut self) -> Option<ASTNode> {
        let mut result = self.term()?;

        while let Some(token) = self.current_token.clone() {
            if !['+', '-'].map(|c| String::from(c)).contains(&token.value) {
                break;
            }

            if token._type != TokenType::Operator {
                break;
            }

            self.step();

            result = ASTNode::BinOp { 
                left: Box::new(result), 
                op: token.value, 
                right: Box::new(self.expr().unwrap())
            };
        }

        Some(result)
    }

    pub fn parse(&mut self) -> Option<ASTNode> {
        self.expr()
    }
}


#[test]
fn expr_plus_term_grammar() {
    let mut parser = Parser::new(&"2+2");

    assert_eq!(
        parser.parse().unwrap(),
        ASTNode::BinOp { 
            left: Box::new(ASTNode::Number(2)), 
            op: "+".to_string(), 
            right: Box::new(ASTNode::Number(2))
        }
    );
}

#[test]
#[should_panic]
fn parse_expr_failed() {
    Parser::new("2+").parse();
}

#[test]
#[should_panic]
fn parse_term_failed() {
    Parser::new("2*").parse();
}

#[test]
fn fact_grammar_number() {
    let mut parser = Parser::new(&"2");

    assert_eq!(
        parser.parse().unwrap(),
        ASTNode::Number(2)
    )
}

#[test]
fn fact_grammar_unary_op_negative() {
    let mut parser = Parser::new(&"-(2)");

    assert_eq!(
        parser.parse().unwrap(),
        ASTNode::UnaryOp { 
            op: "-".to_string(), 
            value: Box::new(ASTNode::Number(2))
        }
    )
}

#[test]
fn fact_grammar_unary_op_positive() {
    let mut parser = Parser::new(&"+(2)");

    assert_eq!(
        parser.parse().unwrap(),
        ASTNode::UnaryOp { 
            op: "+".to_string(), 
            value: Box::new(ASTNode::Number(2))
        }
    )
}

#[test]
#[should_panic(expected="[grammar] Неожиданный оператор '*'!")]
fn fact_grammar_unary_op_failure() {
    let mut parser = Parser::new(&"*(2)");
    parser.parse();
}

#[test]
fn term_grammar_mul_binary_op() {
    let mut parser = Parser::new(&"2*2");

    assert_eq!(
        parser.parse().unwrap(),
        ASTNode::BinOp {
            left: Box::new(ASTNode::Number(2)),
            op: "*".to_string(),
            right: Box::new(ASTNode::Number(2))
        }
    )
}

#[test]
fn expr_grammar_plus_binary_op() {
    let mut parser = Parser::new(&"2+2");

    assert_eq!(
        parser.parse().unwrap(),
        ASTNode::BinOp {
            left: Box::new(ASTNode::Number(2)),
            op: "+".to_string(),
            right: Box::new(ASTNode::Number(2))
        }
    )
}

#[test]
fn empty_code() {
    let mut parser = Parser::new(&"");
    assert_eq!(None, parser.parse());
}

#[test]
#[should_panic(expected="[grammar] Неожиданный токен RParen ')'!")]
fn unexpected_token() {
    let mut parser = Parser::new(&")");
    parser.parse();
}

#[test]
#[should_panic(expected="[grammar] Ожидался токен c типом RParen, получен тип LParen")]
fn fact_grammar_unexpected_token() {
    let mut parser = Parser::new(&"2+(2+2(");
    parser.parse();
}

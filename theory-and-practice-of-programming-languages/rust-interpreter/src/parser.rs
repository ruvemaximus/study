#[path = "lexer.rs"]
mod lexer;

use lexer::{Token, TokenType, Lexer};


#[derive(Debug)]
pub enum ASTNode {
    Number(i32),
    BinOp { left: Box<Self>, op: String, right: Box<Self> }
}

pub struct Parser {
    lexer: lexer::Lexer,
    current_token: Option<Token>
}


#[allow(unused_variables, dead_code)]
impl Parser {
    pub fn new(code: &str) -> Self {
        let mut lexer = Lexer::new(&code);
        let current_token = lexer.next_token();

        Self { current_token, lexer }
    }

    fn check_token(&mut self, expected: TokenType) {
        if let Some(token) = &self.current_token {
            if token._type != expected {
                panic!("[grammar] Ожидался токен {expected:?}, получен {token:?}")
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
                TokenType::LParen => todo!(),
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
                left: Box::new(self.term()?), 
                op: token.value, 
                right: Box::new(result)
            };
        }
        Some(result)
    }

    fn expr(&mut self) -> Option<ASTNode> {
        let mut result = self.term()?;

        while let Some(token) = self.current_token.clone() {
            dbg!(&token);
            if !['+', '-'].map(|c| String::from(c)).contains(&token.value) {
                break;
            }

            if token._type != TokenType::Operator {
                break;
            }

            self.step();

            result = ASTNode::BinOp { 
                left: Box::new(self.expr()?), 
                op: token.value, 
                right: Box::new(result)
            };
        }

        Some(result)
    }

    pub fn parse(&mut self) -> Option<ASTNode> {
        self.expr()
    }
}


#[test]
fn fact() {
    let _parser = Parser::new(&"2+2+2");
    assert_eq!(1, 1);
}



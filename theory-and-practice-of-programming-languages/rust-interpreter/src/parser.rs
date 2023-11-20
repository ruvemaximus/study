#[path = "lexer.rs"]
mod lexer;

use lexer::{Token, TokenType};


#[derive(Debug)]
pub enum ASTNode {
    Number(i32),
    BinOp { left: Box<Self>, op: Token, right: Box<Self> }
}

pub struct Parser {
    lexer: lexer::Lexer,
    current_token: Option<Token>
}


impl Parser {
    fn check_token(&mut self, expected: TokenType) {
        if let Some(token) = &self.current_token {
            if token._type != expected {
                panic!("Произошло что-то страшное...")
            }
            self.current_token = self.lexer.next_token();
        }
    }

    fn match_token(&mut self, expected_values: &[String]) -> bool {
        if let Some(Token { value, _type }) = &self.current_token {
            *_type == TokenType::Operator && expected_values.contains(&value)
        } else {
            false
        }
    }

    fn fact(&mut self) -> Option<ASTNode> {
        if let Some(Token { value, _type }) = &self.current_token {
            let node = match _type {
                TokenType::Number => ASTNode::Number(value.parse::<i32>().unwrap()),
                TokenType::LParen => {
                    self.check_token(TokenType::LParen);
                    let result = self.expr();
                    self.check_token(TokenType::RParen);
                    return result;
                },
                _ => panic!("АА ЧТО ПРОИСХОДИТ!")
            };
            Some(node)
        } else { None }
    }

    fn term(&mut self) -> Option<ASTNode> {
        let result = self.fact().unwrap();
        let expected_values = &['*', '/'].map(|ch| String::from(ch));

        while self.match_token(expected_values) {
            if let Some(token) = &self.current_token.clone() {
                self.check_token(TokenType::Operator);
                return Some(ASTNode::BinOp {
                    left: Box::new(result),
                    op: token.clone(),
                    right: Box::new(self.fact().unwrap()) 
                })
            }
        }

        Some(result)
    }


    fn expr(&mut self) -> Option<ASTNode> {
        let mut result = self.term().unwrap();
        let expected_values = &['+', '-'].map(|ch| String::from(ch));

        while self.match_token(expected_values) {
            let token = &self.current_token.clone().unwrap();
            self.check_token(TokenType::Operator);
            result = ASTNode::BinOp { 
                left: Box::new(result), 
                op: token.clone(), 
                right: Box::new(self.term().unwrap())
            }
        }
        Some(result)
    }

    pub fn parse(code: &str) -> ASTNode {
        let mut lexer = lexer::Lexer::new(&code);
        let current_token = lexer.next_token();

        let mut parser = Self { current_token, lexer };

        parser.expr().unwrap()
    }
}



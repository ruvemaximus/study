#[path = "lexer.rs"]
mod lexer;

use lexer::{Token, TokenType, Lexer};


#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Number(i32),
    BinOp { left: Box<Self>, op: String, right: Box<Self> },
    UnaryOp { op: String, value: Box<Self> },
    Assignment { variable: String, expr: Box<Self> },
    StmtList { stmt: Box<Self>, rest: Box<Self> },
    Variable(String),
    Empty
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
                },
                TokenType::ID => {
                    self.step();
                    ASTNode::Variable(v.to_string())
                }
                _ => panic!("[grammar] Неожиданный токен {t:?} '{v}'!")
            };

            return Some(node);
        }
        None
    }

    fn assignment(&mut self) -> Option<ASTNode> {
        let Token {_type: _, value: v} = self.current_token.clone().unwrap();

        self.check_token(TokenType::ID);
        self.check_token(TokenType::Assign);

        Some(ASTNode::Assignment { 
            variable: v.to_string(),
            expr: Box::new(self.expr().unwrap()) 
        })
    }

    fn term(&mut self) -> Option<ASTNode> {
        let mut result = self.fact()?;

        while let Some(token) = self.current_token.clone() {
            if !['*', '/'].map(|c| String::from(c)).contains(&token.value) {
                break;
            }

            self.check_token(TokenType::Operator);

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

            self.check_token(TokenType::Operator);

            result = ASTNode::BinOp { 
                left: Box::new(result), 
                op: token.value, 
                right: Box::new(self.expr().unwrap())
            };
        }

        Some(result)
    }

    fn stmt(&mut self) -> Option<ASTNode> {
        let Token { _type: t, value: v } = self.current_token.as_ref().unwrap();

        match t {
            TokenType::ID => self.assignment(),
            TokenType::Begin => self.complex_stmt(),
            TokenType::End => Some(ASTNode::Empty),
            _ => panic!("[grammar] Неожиданный токен {t:?} '{v}'!")
        }
    }

    fn stmts(&mut self) -> Option<ASTNode> {
        let mut result = self.stmt().unwrap();
        if let Some(token) = &self.current_token {
            if token._type == TokenType::SEMI {
                self.step();
                result = ASTNode::StmtList {
                    stmt: Box::new(result), 
                    rest: Box::new(self.stmts().unwrap())
                }
            }
        }
        Some(result)
    }

    fn complex_stmt(&mut self) -> Option<ASTNode> {
        self.check_token(TokenType::Begin);
        let result = self.stmts().unwrap();
        self.check_token(TokenType::End);
        Some(result)
    }

    fn program(&mut self) -> Option<ASTNode> {
        let result = self.complex_stmt().unwrap();
        self.check_token(TokenType::Dot);
        Some(result)
    }

    pub fn parse(&mut self) -> Option<ASTNode> {
        self.program()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_empty_is_valid() {
        let mut parser = Parser::new(&"BEGIN END.");
        assert_eq!(parser.parse().unwrap(), ASTNode::Empty)
    }

    #[test]
    fn stmts() {
        let mut parser = Parser::new(&"BEGIN a := 1; b := 2 END.");
        assert_eq!(parser.stmts().unwrap(), ASTNode::StmtList { 
            stmt: Box::new(ASTNode::Assignment{variable:"a".to_string(),expr:Box::new(ASTNode::Number(1))}), 
            rest: Box::new(ASTNode::Assignment{variable:"b".to_string(),expr:Box::new(ASTNode::Number(2))})
        });
    }

    #[test]
    #[should_panic(expected="[grammar] Неожиданный токен Number '0'!")]
    fn stmt_unexpected_token() {
        let mut parser = Parser::new(&"0");
        parser.stmt();
    }

    #[test]
    fn stmt_token_end() {
        let mut parser = Parser::new(&"END");
        assert_eq!(parser.stmt().unwrap(), ASTNode::Empty);
    }

    #[test]
    #[should_panic(expected="[grammar] Ожидался токен c типом Number, получен тип Operator")]
    fn check_token_failure() {
        let mut parser = Parser::new(&"+");
        parser.check_token(TokenType::Number);
    }

    #[test]
    fn check_token_empty_is_valid() {
        let mut parser = Parser::new(&"");
        parser.check_token(TokenType::Number);
    }

    #[test]
    fn expr_plus() {
        let mut parser = Parser::new(&"2+2");

        assert_eq!(
            parser.expr().unwrap(),
            ASTNode::BinOp { 
                left: Box::new(ASTNode::Number(2)), 
                op: "+".to_string(), 
                right: Box::new(ASTNode::Number(2))
            }
        );
    }

    #[test]
    #[should_panic]
    fn expr_failed() {
        Parser::new("2+").expr();
    }

    #[test]
    #[should_panic]
    fn term_failed() {
        Parser::new("2*").term();
    }

    #[test]
    fn fact_number() {
        let mut parser = Parser::new(&"2"); 
        assert_eq!(
            parser.fact().unwrap(),
            ASTNode::Number(2)
        )
    }

    #[test]
    fn fact_variable() {
        let mut parser = Parser::new(&"x"); 
        assert_eq!(
            parser.fact().unwrap(),
            ASTNode::Variable("x".to_string())
        );
    }

    #[test]
    fn fact_unary_op_negative() {
        let mut parser = Parser::new(&"-(2)");

        assert_eq!(
            parser.fact().unwrap(),
            ASTNode::UnaryOp { 
                op: "-".to_string(), 
                value: Box::new(ASTNode::Number(2))
            }
        )
    }

    #[test]
    fn fact_unary_op_positive() {
        let mut parser = Parser::new(&"+(2)");

        assert_eq!(
            parser.fact().unwrap(),
            ASTNode::UnaryOp { 
                op: "+".to_string(), 
                value: Box::new(ASTNode::Number(2))
            }
        )
    }

    #[test]
    #[should_panic(expected="[grammar] Неожиданный оператор '*'!")]
    fn fact_unary_op_unexprected_token() {
        let mut parser = Parser::new(&"*(2)");
        parser.fact();
    }

    #[test]
    fn term_mul_binary_op() {
        let mut parser = Parser::new(&"2*2");

        assert_eq!(
            parser.term().unwrap(),
            ASTNode::BinOp {
                left: Box::new(ASTNode::Number(2)),
                op: "*".to_string(),
                right: Box::new(ASTNode::Number(2))
            }
        )
    }

    #[test]
    fn expr_plus_binary_op() {
        let mut parser = Parser::new(&"2+2");

        assert_eq!(
            parser.expr().unwrap(),
            ASTNode::BinOp {
                left: Box::new(ASTNode::Number(2)),
                op: "+".to_string(),
                right: Box::new(ASTNode::Number(2))
            }
        )
    }

    #[test]
    fn expr_empty_is_valid() {
        let mut parser = Parser::new(&"");
        assert_eq!(None, parser.expr());
    }

    #[test]
    #[should_panic(expected="[grammar] Неожиданный токен RParen ')'!")]
    fn expr_unexpected_token() {
        let mut parser = Parser::new(&")");
        parser.expr();
    }
}

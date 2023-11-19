#[path = "lexer.rs"]
mod lexer;
use lexer::Token;


pub enum ASTNode {
    Number(i32),
    BinOp { left: Box<Self>, op: Token, right: Box<Self> }
}

pub struct Parser {
    lexer: lexer::Lexer,
    current_token: Token
}


impl Parser {
    fn check_token(&mut self, expected: Token) {
        Token::compare_types(&self.current_token, &expected);
        self.current_token = self.lexer.next_token().unwrap();
    }

    fn fact(&mut self) -> ASTNode {
        match self.current_token {
            Token::Number(value) => ASTNode::Number(value),
            Token::OpenParen => {
                self.check_token(Token::OpenParen);
                let result = self.expr();
                self.check_token(Token::CloseParen);
                return result;
            },
            _ => panic!("АА ЧТО ПРОИСХОДИТ!")
        }
    }

    fn term(&mut self) -> ASTNode {
        let result = self.fact();
        
        result
    }


    fn expr(&mut self) -> ASTNode {
        let result = self.term();
        while [
            Token::Operator('/'),
            Token::Operator('*')
        ].contains(&self.current_token) {
            let token = &self.current_token;
            self.check_token(Token::Operator('\0'));
        }
        result
    }

    pub fn parse(code: &str) -> ASTNode {
        let mut lexer = lexer::Lexer::new(&code);
        let current_token = lexer.next_token().unwrap();

        let mut parser = Self { current_token, lexer };

        parser.expr()
    }
}



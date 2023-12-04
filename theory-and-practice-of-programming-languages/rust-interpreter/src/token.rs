#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number,
    Operator,
    LParen,
    RParen,
    Assign,
    Begin,
    End, 
    ID,
    SEMI
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub _type: TokenType,
    pub value: std::string::String
}

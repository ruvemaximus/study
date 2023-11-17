enum Expr {
    BinOp { left: Box<Expr>, op: lexer::Token, right: Box<Expr> }
}

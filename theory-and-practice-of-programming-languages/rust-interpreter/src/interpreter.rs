#[path = "parser.rs"]
mod parser;
use parser::{Parser, ASTNode};


fn visit(node: &ASTNode) -> i32 {
    match node {
        ASTNode::Number(v) => *v,
        ASTNode::BinOp { left, op, right } => {
            match op.as_str() {
                "*" => visit(left) * visit(right),
                "/" => visit(left) / visit(right),
                "+" => visit(left) + visit(right),
                "-" => visit(left) - visit(right),
                op @ _ => {
                    panic!("[interpreter] Неожиданная операция '{op}'")
                }
            }
        }
    }
}

pub fn eval(code: &str) -> i32 {
    let mut parser = Parser::new(code);

    visit(&parser.parse().unwrap())
}


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
                // Это неизвестный оператор или ошибка в грамматике, на это ругнется `lexer` или
                // `parser` соответственно
                _ => unreachable!()
            }
        },
        ASTNode::UnaryOp { op, value } => {
            match op.as_str() {
                "+" => visit(value),
                "-" => -visit(value),
                _ => unreachable!()
            }
        }
    }
}

pub fn eval(code: &str) -> i32 {
    let mut parser = Parser::new(code);

    visit(&parser.parse().unwrap())
}


#[test]
fn eval_unary_op_positive() {
    assert_eq!(eval("+(2)"), 2);
}

#[test]
fn eval_unary_op_negative() {
    assert_eq!(eval("-(2)"), -2);
}


#[test]
fn eval_sum() {
    assert_eq!(eval("2+3"), 5);
}

#[test]
fn eval_sub() {
    assert_eq!(eval("3-1"), 2);
}

#[test]
fn eval_div() {
    let result = eval(&"10/2");
    assert_eq!(result, 5);
}

#[test]
fn eval_complex_expression() {
    let result = eval("2 * (3 + 4) - 5");
    assert_eq!(result, 9);
}

#[test]
fn eval_mul() {
    let result = eval(&"10*2");
    assert_eq!(result, 20);
}

#[test]
fn number() {
    assert_eq!(eval("2"), 2);
}


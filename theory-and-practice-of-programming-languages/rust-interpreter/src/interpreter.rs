#[path = "parser.rs"]
mod parser;

use parser::{Parser, ASTNode};
use std::collections::HashMap;

pub struct Interpreter {
    pub variables: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }
    fn visit(&mut self, node: &ASTNode) -> i32 {
        match node {
            ASTNode::Number(v) => *v,
            ASTNode::BinOp { left, op, right } => {
                match op.as_str() {
                    "*" => self.visit(left) * self.visit(right),
                    "/" => self.visit(left) / self.visit(right),
                    "+" => self.visit(left) + self.visit(right),
                    "-" => self.visit(left) - self.visit(right),
                    _ => unreachable!("Парсер гарантирует отсутствие других операндов")
                }
            },
            ASTNode::UnaryOp { op, value } => {
                match op.as_str() {
                    "+" => self.visit(value),
                    "-" => -self.visit(value),
                    _ => unreachable!("Парсер гарантирует отсутствие других операндов")
                }
            },
            ASTNode::StmtList { stmt, rest } => {
                self.visit(stmt);
                self.visit(rest)
            },
            ASTNode::Assignment { variable, expr } => {
                let rvalue = self.visit(expr);
                self.variables.insert(variable.to_string(), rvalue);
                self.variables[variable]
            },
            ASTNode::Variable(variable) => self.variables[variable],
            ASTNode::Empty => 0 
        }
    }

    pub fn eval(&mut self, code: &str) -> i32 {
        let mut parser = Parser::new(code);
        self.visit(&parser.parse().unwrap())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visit_number() {
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.visit(&ASTNode::Number(2)),
            2
        )
    }

    #[test]
    fn visit_ast_empty() {
        let mut interpreter = Interpreter::new();
        assert_eq!(interpreter.visit(&ASTNode::Empty), 0);
    }

    #[test]
    fn visit_stmt_list() {
        let mut interpreter = Interpreter::new();
        assert_eq!(
            interpreter.visit(&ASTNode::StmtList { 
                stmt: Box::new(ASTNode::Number(1)), 
                rest: Box::new(ASTNode::Number(2)) 
            }),
            2
        );
    }

    #[test]
    fn visit_variable() {
        let mut interpreter = Interpreter::new();
        interpreter.eval("BEGIN x := 2 END.");
        assert_eq!(
            interpreter.visit(&ASTNode::Variable("x".to_string())),
            2
        );
    }

    #[test]
    fn reassign_variable() {
        let mut interpreter = Interpreter::new();
        interpreter.eval("BEGIN x := 2; x := 3 END.");
        assert_eq!(
            interpreter.visit(&ASTNode::Variable("x".to_string())),
            3
        );
    }

    fn unary_op(op: &str, value: i32) -> ASTNode {
        ASTNode::UnaryOp { 
            op: op.to_string(), 
            value: Box::new(ASTNode::Number(value)) 
        }
    }

    #[test]
    fn visit_unary_op_negative() {
        let mut interpreter = Interpreter::new();
        let ast = unary_op("-", 2);
        assert_eq!(interpreter.visit(&ast), -2);
    }

    #[test]
    fn visit_unary_op_positive() {
        let mut interpreter = Interpreter::new();
        let ast = unary_op("+", 2);
        assert_eq!( interpreter.visit(&ast), 2);
    }

    fn bin_op(left: i32, op: &str, right: i32) -> ASTNode {
        ASTNode::BinOp { 
            left: Box::new(ASTNode::Number(left)), 
            op: op.to_string(), 
            right: Box::new(ASTNode::Number(right))
        }
    }

    #[test]
    fn visit_bin_op_sum() {
        let mut interpreter = Interpreter::new();
        assert_eq!(interpreter.visit(&bin_op(2, "+", 3)), 5);
    }

    #[test]
    fn visit_bin_op_sub() {
        let mut interpreter = Interpreter::new();
        assert_eq!(interpreter.visit(&bin_op(2, "-", 3)), -1);
    }

    #[test]
    fn visit_bin_op_div() {
        let mut interpreter = Interpreter::new();
        assert_eq!(interpreter.visit(&bin_op(10, "/", 5)), 2);
    }

    #[test]
    fn visit_bin_op_mul() {
        let mut interpreter = Interpreter::new();
        assert_eq!(interpreter.visit(&bin_op(6, "*", 2)), 12);
    }

    #[test]
    fn eval_complex_expression() {
        let mut interpreter = Interpreter::new();
        interpreter.eval("BEGIN x:=2 * (3 + 4) - 5 END.");
        assert_eq!(interpreter.variables["x"], 9);
    }
}

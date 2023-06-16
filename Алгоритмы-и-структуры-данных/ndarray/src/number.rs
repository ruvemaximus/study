use std::cmp::Ordering;
use std::iter::Sum;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Number { 
    Float(f32),
    Int(i32)
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Number::Float(a) => match other {
                Number::Float(b) => {
                    if a > b { Ordering::Greater } else { Ordering::Less }
                },
                Number::Int(b) => {
                    if *a > *b as f32 { Ordering::Greater } else { Ordering::Less }
                }
            },
            Number::Int(a) => match other {
                Number::Float(b) => {
                    if *a as f32 > *b { Ordering::Greater } else { Ordering::Less }
                },
                Number::Int(b) => {
                    if a > b { Ordering::Greater } else { Ordering::Less }
                }
            },
        }
        
    }
}

pub enum Operation { ADD, SUB, MUL, DIV }


fn calc_number(operation: Operation, left: Number, right: Number) -> Number { 
    let num: Number;

    match left {
        Number::Int(a) => match right {
            Number::Float(b) => match operation {
                Operation::ADD => num = Number::Float(a as f32 + b),
                Operation::SUB => num = Number::Float(a as f32 - b),
                Operation::MUL => num = Number::Float(a as f32 * b), 
                Operation::DIV => num = Number::Float(a as f32 / b)
            },
            Number::Int(b) =>  match operation {
                Operation::ADD => num = Number::Int(a + b), 
                Operation::SUB => num = Number::Int(a - b),
                Operation::MUL => num = Number::Int(a * b), 
                Operation::DIV => num = Number::Int(a / b)
            }
        },
        Number::Float(a) =>  match right {
            Number::Float(b) => match operation {
                Operation::ADD => num = Number::Float(a + b), 
                Operation::SUB => num = Number::Float(a - b),
                Operation::MUL => num = Number::Float(a * b), 
                Operation::DIV => num = Number::Float(a / b)
            },
            Number::Int(b) => match operation {
                Operation::ADD => num = Number::Float(a + b as f32), 
                Operation::SUB => num = Number::Float(a - b as f32),
                Operation::MUL => num = Number::Float(a * b as f32), 
                Operation::DIV => num = Number::Float(a / b as f32)
            }   
        }
    };
    num
}

impl Add for Number { 
    type Output = Number;

    fn add(self, other: Number) -> Number { 
        calc_number(Operation::ADD, self, other)
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        calc_number(Operation::SUB, self, other)
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        calc_number(Operation::MUL, self, other)
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        calc_number(Operation::DIV, self, other)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::Int(val) => write!(f, "{}", val),
            Number::Float(val) => write!(f, "{}", val)
        }
    }
}

impl Eq for Number { }

pub fn sum(arr: Vec<Number>) ->  usize { 
    let mut a = Number::Int(0);
    for i in arr.iter() {
        a = a + *i;
    }
    match a {
        Number::Float(a) => a as usize,
        Number::Int(a) => a as usize
    }
}
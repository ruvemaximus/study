use std::io;


fn prefix_to_infix(expression: &str) -> String {
    let expression = expression.split(' ').rev();
    let mut stack: Vec<String> = Vec::new();

    for token in expression {
        match token {
            "+" | "-" | "*" | "/" => { 
                let a = stack.pop();
                let b = stack.pop();

                stack.push(
                    match (a, b) {
                        (Some(_a), Some(_b)) => format!("({_a} {token} {_b})"),
                        (_, _) => panic!("Строка невалидна!")
                    }
                );
            },
            _ => stack.push(String::from(token)),
        };
    }

    stack[0].clone()
}


fn main() { 
    let mut expression = String::new();
    println!("Укажите выражение: ");

    io::stdin()
        .read_line(&mut expression)
        .expect("У меня не получилось считать строку :(");

    println!("{}", prefix_to_infix(&expression));
}


#[test]
fn test_prefix_to_infix() {
    let expression = String::from("+ 2 2");
    assert_eq!("(2 + 2)", prefix_to_infix(&expression));
}

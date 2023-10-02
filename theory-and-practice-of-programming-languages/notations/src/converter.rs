pub fn prefix_to_infix(expression: &str) -> String {
    let expression = expression.trim().split(' ').rev();
    let mut stack: Vec<String> = Vec::new();

    for token in expression {
        match token {
            "+" | "-" | "*" | "/" => { 
                let a = stack.pop();
                let b = stack.pop();

                stack.push(
                    match (a, b) {
                        (Some(_a), Some(_b)) => format!("({_a} {token} {_b})"),
                        (_, _) => panic!("Строка невалидна! Слишком много операндов!")
                    }
                );
            },
            number if number.parse::<u8>().is_ok() => stack.push(String::from(token)),
            _  => panic!("Неизвестный токен {}!", token), 
        };
    }

    if stack.len() > 1 {
        panic!("Строка невалидна! Слишком много чисел!");
    }

    stack[0].clone()
}


#[test]
fn test_2_plus_2() {
    assert_eq!(
        "(2 + 2)", 
        prefix_to_infix(&String::from("+ 2 2"))
    );
}

#[test]
fn test_pattern_1() {
    assert_eq!(
        prefix_to_infix(&String::from("+ 2 * 2 - 2 1")),
        "(2 + (2 * (2 - 1)))"
    );
}

#[test]
fn test_pattern_5() {
    assert_eq!(
        "((3 + 10) / ((2 + 3) * (3 - 5)))", 
        prefix_to_infix(&String::from("/ + 3 10 * + 2 3 - 3 5"))
    );
}

#[test]
#[should_panic]
fn test_too_many_tokens() {
    prefix_to_infix(&String::from("+ + 2 2"));
}

#[test]
#[should_panic]
fn test_too_many_numbers() {
    prefix_to_infix(&String::from("2 2"));
}

#[test]
#[should_panic]
fn test_unexpected_token() {
    prefix_to_infix(&String::from("a"));
}

#[test]
#[should_panic]
fn test_empty_string() {
    prefix_to_infix(&String::from(""));
}

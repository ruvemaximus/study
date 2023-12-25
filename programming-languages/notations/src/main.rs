use std::io;
mod converter;


fn main() { 
    let mut expression = String::new();
    println!("Укажите выражение: ");

    io::stdin()
        .read_line(&mut expression)
        .expect("У меня не получилось считать строку :(");

    println!("{}", converter::prefix_to_infix(&expression));
}


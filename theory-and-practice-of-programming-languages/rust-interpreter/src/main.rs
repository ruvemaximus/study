mod lexer;

fn main() {
    let code = "12+2".to_string();
    let tokens = lexer::lexer(&code);

    dbg!(tokens);
}

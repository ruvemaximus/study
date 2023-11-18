#[path = "lexer.rs"]
mod lexer;

pub fn parse(code: &str) {
    let mut lexer = lexer::Lexer::new(&code);

    dbg!(lexer.next_token());
    dbg!(lexer.next_token());
    dbg!(lexer.next_token());
}

mod parser;


fn main() {
    let mut parser = parser::Parser::new(&"2+2*2");
    dbg!(parser.parse());
}

mod parser;


fn main() {
    dbg!(parser::Parser::parse(&" 123 + 456"));
}

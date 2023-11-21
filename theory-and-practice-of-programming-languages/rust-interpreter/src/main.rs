mod interpreter;


fn main() {
    let code = "(2+2)*2";
    println!("{code}={}", interpreter::eval(&code));
}

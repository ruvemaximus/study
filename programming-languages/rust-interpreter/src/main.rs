mod interpreter;
use interpreter::Interpreter;


fn main() {
    let mut super_interpreter_written_on_best_programing_language = Interpreter::new();
    let code = r#"
BEGIN
    y: = 2;
    BEGIN
        a := 3;
        a := a;
        b := 10 + a + 10 * y / 4;
        c := a - b
    END;
    x := 11;
END.
    "#;
    super_interpreter_written_on_best_programing_language.eval(&code);
    dbg!(super_interpreter_written_on_best_programing_language.variables);
}

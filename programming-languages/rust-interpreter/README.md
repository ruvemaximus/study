# Интепретатор на языке Rust
## Грамматика
```
<expr> ::= <term> | <term> + <expr> | <term> - <expr>
<term> ::= <fact> | <fact> * <term> | <fact> / <term>
<fact> ::= <number> | ( <expr> ) | - ( <expr> ) | + ( <expr> )
```

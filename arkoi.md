## Operators

| Name       | Operators | Associates |
|------------|-----------|------------|
| Equality   | == !=     | Left       |
| Comparison | > >= < <= | Left       |
| Term       | - +       | Left       |
| Factor     | / *       | Left       |
| Unary      | ! -       | Right      |


## EBNF
```ebnf
program = declaration* EOF ;

declaration = fun_declaration 
            | let_declaration
            | statement ;

fun_declaration = "fun" IDENTIFIER "(" parameters? ")" type block ;

parameters = IDENTIFIER type ( "," IDENTIFIER type )* ;

type = "@" ( "u8" | "i8" 
           | "u16" | "i16" 
           | "u32" | "i32" 
           | "u64" | "i64" 
           | "f32" | "f64" 
           | "bool" ) ;

let_declaration = "let" IDENTIFIER ( "=" expression )? ";" ;

statement = expression_statement 
          | block ;

block = "{" declaration* "}" ;

expression_statement = expression ";" ;

expression = equality;

equality = comparison ( ( "==" | "!=" ) comparison )* ;

comparison = term ( ( ">" | ">=" | "<" | "<=" ) term )* ;

term = factor ( ( "-" | "+" ) factor )* ;

factor = unary ( ( "/" | "*" ) unary )* ;

unary = ( ( "!" | "-" ) unary ) 
      | call ;

call = primary ( "(" arguments? ")" )* ;

arguments = expression ( "," expression )* ;

primary = NUMBER | STRING | IDENTIFIER | "true" | "false" | "(" expression ")" ;
```

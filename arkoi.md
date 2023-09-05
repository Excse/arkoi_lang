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
program = statement* EOF ;

declaration = let_declaration
            | statement ; 

let_declaration = "let" IDENTIFIER ( "=" expression )? ";" ;

statement = expression_statement ;

expression_statement = expression ";" ;

expression = equality;

equality = comparison ( ( "==" | "!=" ) comparison )* ;

comparison = term ( ( ">" | ">=" | "<" | "<=" ) term )* ;

term = factor ( ( "-" | "+" ) factor )* ;

factor = unary ( ( "/" | "*" ) unary )* ;

unary = ( "!" | "-" ) unary
      | primary ;

primary = NUMBER | STRING | IDENTIFIER | BOOLEAN | "(" expression ")" ;
```

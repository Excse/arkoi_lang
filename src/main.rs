use lexer;

use lexer::token::TokenKind;
use lexer::Lexer;

fn main() {
    let source_code = match std::fs::read_to_string("examples/struct.ark") {
        Ok(code) => code,
        Err(err) => panic!("{err}"),
    };

    let mut lexer = Lexer::new(&source_code);
    let mut tokens = lexer.tokenize();
    tokens.retain(|token| !matches!(token, TokenKind::Whitespace));

    println!("{tokens:?}");
}

use diagnostics::SourceDetails;
use lexer;

use lexer::token::TokenKind;
use lexer::Lexer;

fn main() {
    let source_details = match SourceDetails::read("examples/struct.ark") {
        Ok(source_details) => source_details,
        Err(err) => panic!("{err}"),
    };

    let mut lexer = Lexer::new(&source_details);
    let mut tokens = lexer.tokenize();
    tokens.retain(|token| !matches!(token.kind, TokenKind::Whitespace));

    println!("{tokens:?}");
}

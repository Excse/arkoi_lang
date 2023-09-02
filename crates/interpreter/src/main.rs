use parser::{Parser, ParserError};
use diagnostics::SourceDetails;
use lexer::Lexer;

fn main() {
    let source_details = match SourceDetails::read("examples/parser.ark") {
        Ok(source_details) => source_details,
        Err(err) => panic!("{err}"),
    };

    let mut lexer = Lexer::new(&source_details);
    let mut parser = Parser::new(&mut lexer);

    let statements = parser.parse_program();
    println!("Statements: {:#?}", statements);

    for error in parser.errors {
        match error {
            ParserError::Diagnostic(report) => println!("{}", report),
            error => println!("{:#?}", error),
        }
    }
}

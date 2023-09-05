mod execute;

use diagnostics::SourceDetails;
use execute::Interpreter;
use lexer::Lexer;
use parser::{traversel::Visitable, Parser, ParserError};

fn main() {
    let source_details = match SourceDetails::read("examples/parser.ark") {
        Ok(source_details) => source_details,
        Err(err) => panic!("{err}"),
    };

    let mut lexer = Lexer::new(&source_details);
    let mut parser = Parser::new(&mut lexer);

    let statements = parser.parse_program();

    if !parser.errors.is_empty() {
        println!("Statements: {:#?}", statements);

        for error in parser.errors {
            match error {
                ParserError::Diagnostic(report) => println!("{}", report),
                error => println!("{:#?}", error),
            }
        }

        return;
    }

    let mut interpreter = Interpreter;
    statements.iter().for_each(|statement| {
        let result = statement.accept::<Interpreter>(&mut interpreter);
        println!("{:#?} with a result of {:?}", statement, result);
    });
}

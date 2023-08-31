use diagnostics::SourceDetails;

use parser::Parser;
use lexer::Lexer;

fn main() {
    let source_details = match SourceDetails::read("examples/parser.ark") {
        Ok(source_details) => source_details,
        Err(err) => panic!("{err}"),
    };

    let mut lexer = Lexer::new(&source_details);
    let mut parser = Parser::new(&mut lexer);

    let expression = parser.parse_expression().unwrap();
    println!("{:#?}", expression);
}

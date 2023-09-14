mod execute;

use diagnostics::{file::Files, renderer::Renderer};
use execute::Interpreter;
use lasso::Rodeo;
use lexer::{Lexer, LexerError};
use parser::{traversel::Visitable, Parser, ParserError};
use termcolor::{ColorChoice, StandardStream};

fn main() {
    let mut files = Files::default();

    let path = "examples/parser.ark";
    let source = std::fs::read_to_string(path).expect("Couldn't read the file.");
    let file_id = files.add(path, &source);

    let stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut renderer = Renderer::new(&files, stdout);

    let mut interner = Rodeo::new();

    let mut lexer = Lexer::new(&files, file_id, &mut interner);
    if !lexer.errors.is_empty() {
        for error in lexer.errors {
            match error {
                LexerError::Diagnostic(report) => renderer.render(&report),
                error => println!("{:#?}", error),
            }
        }

        return;
    }

    let mut parser = Parser::new(&files, file_id, &mut lexer);
    let statements = parser.parse_program();

    if !parser.errors.is_empty() {
        for error in parser.errors {
            match error {
                ParserError::Report(report) => renderer.render(&report),
                error => println!("{:#?}", error),
            }
        }

        return;
    }

    // let mut name_resolution = NameResolution::new();
    // statements.iter().for_each(|statement| {
    //     let _ = statement.accept::<NameResolution>(&mut name_resolution);
    // });

    let mut interpreter = Interpreter::new(&mut interner);
    statements.iter().for_each(|statement| {
        let result = statement.accept::<Interpreter>(&mut interpreter);
        println!("{:#?} with a result of {:?}", statement, result);
    });
}

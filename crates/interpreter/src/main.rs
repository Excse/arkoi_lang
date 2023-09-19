#![allow(unused)]

mod execute;

use lasso::Rodeo;
use name_resolution::{NameResolution, ResolutionError};
use termcolor::{ColorChoice, StandardStream};

use diagnostics::{file::Files, renderer::Renderer};
use execute::Interpreter;
use lexer::{error::LexerError, Lexer};
use parser::{
    traversel::{Visitable, Visitor},
    Parser,
};

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
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        for error in parser.errors {
            match error {
                // ParserError::Report(report) => renderer.render(&report),
                error => println!("{:#?}", error),
            }
        }

        return;
    }

    let mut name_resolution = NameResolution::default();
    name_resolution.visit_program(&program);

    if !name_resolution.errors.is_empty() {
        for error in name_resolution.errors {
            match error {
                ResolutionError::Report(report) => renderer.render(&report),
                error => println!("{:#?}", error),
            }
        }

        return;
    }

    let mut interpreter = Interpreter::new(&mut interner);
    interpreter.visit_program(&program);
}

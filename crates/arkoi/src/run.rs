use std::path::PathBuf;

use clap::Args;
use lasso::Rodeo;
use parser::Parser;
use termcolor::{ColorChoice, StandardStream};

use ast::traversal::Visitor;
use diagnostics::{file::Files, renderer::Renderer};
use interpreter::Interpreter;
use lexer::{error::LexerError, Lexer};
use name_resolution::{error::ResolutionError, NameResolution};

#[derive(Args)]
pub struct RunArgs {
    // The file that should be run
    input_file: PathBuf,
}

pub fn run(args: RunArgs) {
    let input_path = args.input_file.as_path();
    if !input_path.exists() {
        panic!("The input file doesn't exist.");
    }

    let source = std::fs::read_to_string(input_path).expect("Couldn't read the file.");
    let input_path = input_path.to_string_lossy();

    let mut files = Files::new();
    let file_id = files.add(input_path, &source);

    let mut interner = Rodeo::new();

    let mut lexer = Lexer::new(&files, file_id, &mut interner);
    if !lexer.errors.is_empty() {
        let stdout = StandardStream::stdout(ColorChoice::Auto);
        let mut renderer = Renderer::new(&files, stdout);

        for error in lexer.errors {
            match error {
                LexerError::Diagnostic(report) => renderer.render(&report),
                error => println!("{:#?}", error),
            }
        }

        return;
    }

    let mut parser = Parser::new(&files, file_id, &mut lexer);
    let mut program = parser.parse_program();

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
    name_resolution.visit_program(&mut program);

    if !name_resolution.errors.is_empty() {
        let stdout = StandardStream::stdout(ColorChoice::Auto);
        let mut renderer = Renderer::new(&files, stdout);

        for error in name_resolution.errors {
            match error {
                ResolutionError::Report(report) => renderer.render(&report),
                error => println!("{:#?}", error),
            }
        }

        return;
    }

    let mut interpreter = Interpreter::new(&mut interner);
    program.statements.iter_mut().for_each(|statement| {
        println!("{:?}", interpreter.visit_statement(statement));
    });
}

use std::path::PathBuf;

use clap::Args;
use lasso::Rodeo;
use semantics::Semantics;
use termcolor::{ColorChoice, StandardStream};

use ast::traversal::Visitor;
use diagnostics::{file::Files, renderer::Renderer};
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

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

    let stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut renderer = Renderer::new(&files, stdout);
    let mut interner = Rodeo::new();

    let lexer = Lexer::new(&files, file_id, &mut interner);
    if !lexer.errors.is_empty() {
        for error in lexer.errors {
            renderer.render(error);
        }

        return;
    }

    let iterator = lexer.into_iter();
    let mut parser = Parser::new(iterator);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        for error in parser.errors {
            renderer.render(error);
        }

        return;
    }

    let mut semantics = Semantics::new(&program);
    semantics.run_all();

    if !semantics.errors.is_empty() {
        for error in semantics.errors {
            renderer.render(error);
        }

        return;
    }

    let mut interpreter = Interpreter::new(&mut interner);
    program.statements.iter().for_each(|statement| {
        println!("{:?}", interpreter.visit_stmt(statement));
    });
}

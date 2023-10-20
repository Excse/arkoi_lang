use std::{cell::RefCell, path::PathBuf, rc::Rc};

use clap::Args;
use lasso::Rodeo;
use termcolor::{ColorChoice, StandardStream};

use diagnostics::{file::Files, renderer::Renderer};
use lexer::Lexer;
use parser::Parser;
use semantics::Semantics;

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
    let interner = Rc::new(RefCell::new(Rodeo::new()));
    let mut renderer = Renderer::new(&files, interner.clone(), stdout);

    let lexer = Lexer::new(&files, file_id, interner.clone());
    if !lexer.errors.is_empty() {
        for error in lexer.errors {
            renderer.render(error);
        }

        return;
    }

    let iterator = lexer.into_iter();
    let mut parser = Parser::new(iterator);
    let mut program = parser.parse_program();

    if !parser.errors.is_empty() {
        for error in parser.errors {
            renderer.render(error);
        }

        return;
    }

    let mut semantics = Semantics::new(&mut program);
    semantics.run_all();

    if !semantics.errors.is_empty() {
        for error in semantics.errors {
            renderer.render(error);
        }
    }
}

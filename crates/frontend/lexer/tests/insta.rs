#[cfg(feature = "serialize")]
use serde::Serialize;

use std::cell::RefCell;
use std::rc::Rc;

use lasso::Rodeo;

use diagnostics::file::Files;
use lexer::{token::Token, Lexer};

macro_rules! insta_test {
    ($name:ident, $path:expr) => {
        #[derive(Serialize)]
        struct InstaSnapshot<'a> {
            tokens: &'a [Token],
            interner: &'a Rodeo,
        }

        #[test]
        fn $name() {
            let mut files = Files::default();

            let source = std::fs::read_to_string($path).expect("Couldn't read the file.");
            let file_id = files.add($path, &source);

            let interner = Rc::new(RefCell::new(Rodeo::default()));

            let lexer = Lexer::new(&files, file_id, interner.clone());
            let iterator = lexer.into_iter();
            let tokens = iterator.collect::<Vec<Token>>();

            insta::assert_yaml_snapshot!(InstaSnapshot {
                tokens: &tokens,
                interner: &interner.borrow(),
            });
        }
    };
}

insta_test!(insta_test, "test_files/insta_test.ark");

use std::env;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("coconut.l"); // brings the lexer for `coconut.l` into scope.
lrpar_mod!("coconut.y"); // brings the Parser for `coconut.y` into scope.

use crate::ast;

pub fn parse_str(input: &String) -> Option<Result<Vec<ast::Node>, ()>> {
    let lexer_def = coconut_l::lexerdef(); // Lex the input.
    let lexer = lexer_def.lexer(&input);
    let (res, errs) = coconut_y::parse(&lexer); // Parse the input.
                                                // Check for errors
    for e in errs {
        println!("{}", e.pp(&lexer, &coconut_y::token_epp));
    }
    res
}

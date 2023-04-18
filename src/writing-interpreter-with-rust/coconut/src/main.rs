use std::env;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

// brings the lexer for `calc.l` into scope. By default the module name will be `calc_l`
lrlex_mod!("coconut.l");
// brings the parser for `calc.y` into scope. By default the module name will be `calc_y`
lrpar_mod!("coconut.y");

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = &args[1];
        // Create a lexer
        let lexer_def = coconut_l::lexerdef();
        // Lex the input.
        let lexer = lexer_def.lexer(&input);
        // Parse the input.
        let (res, errs) = coconut_y::parse(&lexer);
        // Check for errors
        for e in errs {
            println!("{}", e.pp(&lexer, &coconut_y::token_epp));
        }
        // Print results
        match res {
            Some(Ok(r)) => println!("{:?}", r),
            _ => eprintln!("Unable to evaluate expression."),
        }
    } else {
        println!("Please provide at least one cli argument!")
    }
}

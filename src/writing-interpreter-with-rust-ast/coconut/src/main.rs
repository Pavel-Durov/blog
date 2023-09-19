use std::env;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("coconut.l"); // brings the lexer for `coconut.l` into scope.
lrpar_mod!("coconut.y"); // brings the Parser for `coconut.y` into scope.

mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = &args[1]; // Create a lexer
                              // Print results
        match from_str(input) {
            Ok(r) => println!("{:?}", r),
            _ => eprintln!("Unable to evaluate expression."),
        }
    } else {
        println!("Please provide at least one cli argument!")
    }
}

fn from_str(input: &String) -> Result<u64, String> {
    let lexer_def = coconut_l::lexerdef(); // Lex the input.
    let lexer = lexer_def.lexer(&input);
    let (res, errs) = coconut_y::parse(&lexer); // Parse the input.
                                                // Check for errors
    for e in errs {
        println!("{}", e.pp(&lexer, &coconut_y::token_epp));
    }
    // Print results
    match res {
        Some(Ok(r)) => eval(r),
        _ => Err("Unable to evaluate expression.".to_string()),
    }
}

pub fn eval(ast: Vec<ast::Node>) -> Result<u64, String> {
    for node in ast {
        return eval_exp(node);
    }
    return Err(String::from("Couldn't evaluate given opcodes."));
}

fn eval_exp(exp: ast::Node) -> Result<u64, String> {
    match exp {
        ast::Node::Add { lhs, rhs } => eval_exp(*lhs)?
            .checked_add(eval_exp(*rhs)?)
            .ok_or("overflowed".to_string()),
        ast::Node::Mul { lhs, rhs } => eval_exp(*lhs)?
            .checked_mul(eval_exp(*rhs)?)
            .ok_or("overflowed".to_string()),
        ast::Node::Number { value } => Ok(value),
    }
}

#[test]
fn eval_expressions() {
    assert_eq!(
        from_str(&"0+1*1*1".to_string()).unwrap(),
        1,
        "expected 0+1*1*1"
    );
    assert_eq!(from_str(&"1+1".to_string()).unwrap(), 2, "expected 1+1=2");
    assert_eq!(
        from_str(&"1*(1+2)".to_string()).unwrap(),
        3,
        "expected 1*(1+2)=3"
    );
}

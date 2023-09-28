use std::env;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("coconut.l"); // brings the lexer for `coconut.l` into scope.
lrpar_mod!("coconut.y"); // brings the Parser for `coconut.y` into scope.

mod ast;
mod instruction;

use ast::Node;
use instruction::Op;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = &args[1]; // Create a lexer
        match from_str(input) {
            Ok(Some(r)) => println!("{:?}", r),
            Ok(None) => {}
            _ => eprintln!("Unable to evaluate expression."),
        }
    } else {
        println!("Please provide at least one cli argument!")
    }
}

fn from_str(input: &String) -> Result<Option<u64>, String> {
    let lexer_def = coconut_l::lexerdef(); // Lex the input.
    let lexer = lexer_def.lexer(&input);
    let (res, errs) = coconut_y::parse(&lexer); // Parse the input.
                                                // Check for errors
    for e in errs {
        println!("{}", e.pp(&lexer, &coconut_y::token_epp));
    }
    match res {
        Some(Ok(r)) => Ok(eval(r)),
        _ => Err("Unable to evaluate expression.".to_string()),
    }
}

pub fn eval(ast: Vec<Node>) -> Option<u64> {
    let ops = &mut vec![];
    for a in ast {
        ast_to_bytecode(a, ops);
    }
    let mut stack: Vec<u64> = vec![];
    
    for instruction in ops {
        match instruction {
            Op::Push { value } => stack.push(*value),
            Op::Add => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
            }
            Op::Mull {} => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs * rhs);
            }
        }
    }
    return stack.pop();
}

pub fn ast_to_bytecode(node: Node, ops: &mut Vec<Op>) {
    match node {
        Node::Add { lhs, rhs } => {
            ast_to_bytecode(*lhs, ops);
            ast_to_bytecode(*rhs, ops);
            ops.push(Op::Add {})
        }
        Node::Mul { lhs, rhs } => {
            ast_to_bytecode(*lhs, ops);
            ast_to_bytecode(*rhs, ops);
            ops.push(Op::Mull {})
        }
        Node::Number { value } => ops.push(Op::Push { value }),
    }
}

#[test]
fn eval_expressions() {
    assert_eq!(
        from_str(&"0+1*1*1".to_string()).unwrap(),
        Some(1),
        "expected 0+1*1*1"
    );
    assert_eq!(
        from_str(&"1+1".to_string()).unwrap(),
        Some(2),
        "expected 1+1=2"
    );
    assert_eq!(
        from_str(&"1*(1+2)".to_string()).unwrap(),
        Some(3),
        "expected 1*(1+2)=3"
    );
}

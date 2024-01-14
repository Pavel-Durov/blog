pub mod ast;
pub mod bytecode;
pub mod parser;
pub mod scope;

use bytecode::eval;
use parser::parse_str;
use scope::Scope;

pub fn eval_str(input: &String) -> Result<Option<u64>, String> {
    match parse_str(input) {
        Some(Ok(ast)) => match eval(ast, &mut Scope::new()) {
            Ok(result) => Ok(result),
            Err(x) => Err(x),
        },
        Some(Err(_)) => Err("Unable to parse input.".to_string()),
        _ => Ok(None),
    }
}

use crate::{ast::Node, scope::Scope};

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Mul,
    Assign { name: String },
    Declare { name: String },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,                 // Addition operation
    Mull,                // Multiplication operation
    Push { value: u64 }, // Load numeric value onto stack
    Assign { name: String },
    Declare { name: String },
    PrintLn,
    Load { id: String },
}

pub fn eval(ast: Vec<Node>, scope: &mut Scope) -> Result<Option<u64>, String> {
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
            Op::Assign { name } => {
                let val = stack.pop().unwrap();
                scope.set_var(name.clone(), val);
            }
            Op::Declare { name } => {
                let val = stack.pop().unwrap();
                scope.dec_var(name.clone(), val);
            }
            Op::PrintLn => {
                println!("{}", stack.pop().unwrap());
            }
            Op::Load { id } => {
                if let Some(value) = scope.get_var(id.clone()) {
                    stack.push(value.clone());
                } else {
                    return Err(format!("Variable '{}' not found", id.clone()));
                }
            }
        }
    }
    return Ok(stack.pop());
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
        Node::Declare { id, rhs } => {
            if let Some(val) = rhs {
                ast_to_bytecode(*val, ops);
            }
            ops.push(Op::Declare { name: id.clone() });
        }
        Node::Assign { id, rhs } => {
            ast_to_bytecode(*rhs, ops);
            ops.push(Op::Declare { name: id.clone() });
        }
        Node::Id { value } => ops.push(Op::Load { id: value }),
        Node::PrintLn { rhs } => {
            ast_to_bytecode(*rhs, ops);
            ops.push(Op::PrintLn {})
        }
        Node::Empty {} => {}
    }
}

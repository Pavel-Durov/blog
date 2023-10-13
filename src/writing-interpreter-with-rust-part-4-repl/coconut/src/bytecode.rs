use crate::ast::Node;

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,                 // Addition operation
    Mull,                // Multiplication operation
    Push { value: u64 }, // Load numeric value onto stack
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

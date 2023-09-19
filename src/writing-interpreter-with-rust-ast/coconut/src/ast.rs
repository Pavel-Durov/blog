#[derive(Debug)]
pub enum Opcode {
    Add { lhs: Box<Opcode>, rhs: Box<Opcode> },
    Mul { lhs: Box<Opcode>, rhs: Box<Opcode> },
    Number { value: u64 },
}
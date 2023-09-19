#[derive(Debug)]
pub enum Node {
    Add { lhs: Box<Node>, rhs: Box<Node> },
    Mul { lhs: Box<Node>, rhs: Box<Node> },
    Number { value: u64 },
}

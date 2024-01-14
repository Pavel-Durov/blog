#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Add { lhs: Box<Node>, rhs: Box<Node> },
    Mul { lhs: Box<Node>, rhs: Box<Node> },
    Number { value: u64 },
    Id { value: String },
    PrintLn { rhs: Box<Node> },
    Assign { id: String, rhs: Box<Node> },
    Declare { id: String, rhs: Option<Box<Node>> },
    Empty,
}

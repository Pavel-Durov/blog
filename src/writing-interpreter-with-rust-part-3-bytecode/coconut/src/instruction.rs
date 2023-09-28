#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,  // Addition operation
    Mull, // Multiplication operation
    Push { value: u64 }, // Load numeric value onto stack
}

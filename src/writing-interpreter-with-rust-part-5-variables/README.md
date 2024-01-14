# Writing interpreter in Rust - Variables & Builtins (part-5)

# Abstract

We're continuing our journey of implementing our interpreter called Coconut.

We're going to build on top of the previous functionality of our interpreter. We will extend our Lexer and Parser and implement variable declaration and evaluation as well as a simple built-in function support.

If you haven’t already, I recommend checking out my previous article:
[Writing Interpreter in Rust — Repl, Files, and Comments (Part 4)](https://medium.com/better-programming/writing-interpreter-in-rust-repl-files-and-comments-part-4-5a11d41613ba)


# Introduction

In order to add variables to the Coconut interpreter we will need to add new tokens to our Lexer. The same goes for built-in functions.
We will also implement multi-statement evaluation support, with a ";" termination token - it will be handy when working with variables.

## Lexer

```yacc
%%
[0-9]+ "INTEGER"
\+ "ADD"
\* "MUL"
\( "LPAR"
\) "RPAR"
; ";" 
= "ASSIGN"
let "LET" 
println "PRINT_LN" 
[a-zA-Z0-9_]+ "IDENTIFIER"
[\t\n ]+ ;
//[^\n]*?$ ;
```

`;` - Statment termination token (same as in any C-like language).

`ASSIGN` - Assignment operator. It will assign values to variables.

`LET` - Variable declaration token.

`IDENTIFIER` - Identity token, will be used for variable names.

`PRINT_LN` - Buildin function that will print expression evaluation results to the stdout.


# Parser

Here's what our parser will look like:

```rust
%start StatementList
%%

StatementList -> Result<Vec<Node>, ()>:
    StatementList Statement { append($1.map_err(|_| ())?, $2.map_err(|_| ())?)  }
    | { Ok(vec![]) }
    ;

Statement -> Result<Node, ()>:
   ';' { Ok(Node::Empty{}) }
    | Expression ';' { $1 }
    | Builtins { $1 }
    ;
    
Expression -> Result<Node, ()>:
    AdditiveExpression { $1 }
    | PrimaryExpression 'ASSIGN' Expression {
        match $1.map_err(|_| ())? {
            Node::Id { value } => {
                Ok(Node::Assign { id: value, rhs: Box::new($3?) })
            },
            _ => Err(())
        }
    }
    | 'LET' PrimaryExpression 'ASSIGN' Expression {
        match $2.map_err(|_| ())? {
            Node::Id { value } => {
                Ok(Node::Declare { id: value, rhs: Some(Box::new($4?)) })
            },
            _ => Err(())
        }
    } 
    ;

AdditiveExpression -> Result<Node, ()>:
    MultiplicativeExpression { $1 }
    | AdditiveExpression 'ADD' MultiplicativeExpression { 
        Ok(Node::Add{ lhs: Box::new($1?), rhs: Box::new($3?) })
    }
    ;

MultiplicativeExpression -> Result<Node, ()>: 
    PrimaryExpression { $1 }
    | MultiplicativeExpression 'MUL' PrimaryExpression { 
      Ok(Node::Mul{ lhs: Box::new($1?), rhs: Box::new($3?) })
    }
    ;

PrimaryExpression -> Result<Node, ()>:
    'IDENTIFIER' { Ok(Node::Id { value: $lexer.span_str(($1.map_err(|_| ())?).span()).to_string() }) }
    |  'LPAR' Expression 'RPAR' { $2 }
    | 'INTEGER' { parse_int($lexer.span_str(($1.map_err(|_| ())?).span())) }
    ;

Builtins -> Result<Node, ()>:
    'PRINT_LN' 'LPAR' Expression 'RPAR' { Ok(Node::PrintLn{ rhs: Box::new($3?) }) };

%%
use crate::ast::Node;

fn append(mut lhs: Vec<Node>, rhs: Node ) -> Result<Vec<Node>, ()>{
    lhs.push(rhs);
    Ok(lhs)
}

fn parse_int(s: &str) -> Result<Node, ()> {
    match s.parse::<u64>() {
        Ok(n_val) => Ok(Node::Number{ value: n_val }),
        Err(_) => {
            eprintln!("{} cannot be represented as a u64", s);
            Err(())
        }
    }
}
```

Ast Node enum:
```rust
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
```

It's a bit longer and more complex than what we had before.

The main changes are:

1. Added `Builtins` rule where built-in functions like `println` can be defined.
2. Added PrimaryExpression rule for variable declaration and assignment such as: `let a = 6;`
3. Added support for multiple statements with `;` termination token. Now we can do something like this: `let a = 1; let b = 3; println(a+b);`

And it's structured in a way that we can extend easily later on. We can add new data types like booleans, add new built-in functions etc...


## Evaluation

Generally, the evaluation is done in the same way as before.
We added new Ast nodes to represent our new functionality and we added a notion of a Scope.
Now when we support variables, these variables' identities (aka names) and their values need to be stored somewhere and that somewhere is this Scope.

Our Scope is a simple struct:

```rust
use std::collections::HashMap;

pub struct Scope {
    store: HashMap<String, u64>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            store: HashMap::new(),
        }
    }

    pub fn dec_var(&mut self, id: String, val: u64) -> Option<u64> {
        self.store.insert(id, val)
    }

    pub fn set_var(&mut self, id: String, val: u64) -> Option<u64> {
        self.store.insert(id, val)
    }

    pub fn get_var(&self, id: String) -> Option<&u64> {
        self.store.get(&id.clone())
    }
}
```

Basically, it's a wrapper around the HashMap data structure, so we could've used HashMap directly but I think it creates a nice separation with not much overhead. 
It will also allow us to extend it in the future when we deal with functions, closures and different 
scopes.

With the scop in place, we can evaluate source code line by line and declare, assign and reassign variables.

For example:
```
let a = 1;
let b = 2;
a = 3;
print(a+b); // prints 5
```

### Evaluation

```rust
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
```
Except for the additional scope variable where we can declare and retrieve variables and their values, it should have the same structure as before.

We still using our Stack for evaluation.

### Conversion from AST to Bytecode

Here's our `ast_to_bytecode` function:

```rust
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
```
This change also has no big surprises, this function extended to accompany the new AST nodes and appropriate bytecodes.

## Tests

And of course, let's not forget about our tests.

We added some simple tests to test that our evaluation still works as expected - with and without variables (keeping previous tests in place).

```rust
#[cfg(test)]
#[test]
fn vars_declare_match() {
    assert_eq!(
        eval_str(&"let x = 1; let y = 2; y + x;".to_string()).unwrap(),
        Some(3)
    );
}
#[test]
fn vars_reassign_math() {
    assert_eq!(
        eval_str(&"let x = 1; let y = 2; x = 3; x + y;".to_string()).unwrap(),
        Some(5)
    );
}
#[test]
fn vars_undeclared_variable() {
    assert_eq!(
        eval_str(&"a + 1;".to_string()),
        Err("Variable 'a' not found".to_string())
    );
}
```

We didn't add a test for the `println` functionality, and that's intentional cause I didn't want to add more complexity here with stdout testing - we can explore this in the following posts.


Full source code can be found here: TODO Link.

# Summary

In this article, we've expanded the capabilities of the Coconut interpreter by introducing variable declaration and assignment. Additionally, we've incorporated a simple `println` function, enabling us to output evaluated results to `stdout`.

To facilitate these changes, we introduced a new concept called `Scope`. This allows us to store variable declarations and their corresponding values, with potential extensions planned for the future.

While much of the heavy lifting was handled by the `grmtools` Lexer and Parser, the rest of the implementation followed a straightforward path.

This article serves as a documentation of my own understanding and the organization of my thoughts, aiming to share knowledge with others.

I hope you find it valuable! If you have any further questions or wish to delve into more details, feel free to reach out.

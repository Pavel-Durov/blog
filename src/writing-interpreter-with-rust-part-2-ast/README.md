# Writing an Interpreter in Rust: AST and AST-time evaluation

## Abstract

We're continuing our journey of implementing an interpreter called coconut in Rust! 
If you haven't already, I recommend checking out my previous article here, where we set the stage for our interpreter project. 
Here, we'll delve into the concept of Abstract Syntax Trees (AST) and transition from parse-time evaluation to AST-time evaluation. 
So, fasten your seatbelts, and let's get started!

## What is an AST?

In the previous article, we discussed lexing and parsing, which helped us convert source code into a structured representation. We ended up with something called parse-tree. We never called it by its name or used it explistily but that's where we got to.

An AST is a more refined and abstract representation of the code's syntax. It strips away the extraneous details found in a parse tree, focusing solely on the essential elements required for execution. 


## AST-Time Evaluation

As we mentioned, we used parse-tree, and when we evaluated our computation we did it at parse-time - hense parse-time evaluation.

While this approach works just fine, it's not ideal for more complex programs. 

### AST-time evaluation advantages:

#### Separation of Concerns
With AST, we separate the parsing and evaluation phases. This makes our code more modular and easier to maintain.
#### Optimisations
ASTs allow us to optimise code before execution. Potentially, we can perform various transformations on the tree to improve performance.

#### Error Handling
ASTs provide a clearer structure for error reporting. We can pinpoint errors to specific locations in the code.


So to recap.

**Parse Tree** - includes all the "useless" syntactic information that humans like/need but doesn't affect compilation

**AST** - strip out that useless syntactic stuff, while keeping the esential, structural information.

**Evaluator** - evaluates something (parse tree, AST, or opcodes) directly; a "compiler" converts one thing into another

## Where we ended up

Our starting point is as follows:

Our lexer file `coconut.l`:
```rust
%%
[0-9]+ "INTEGER"
\+ "ADD"
\* "MUL"
\( "LPAR"
\) "RPAR"
[\t\n ]+ ;
```

Our parser file `coconut.y`:


```yacc
%start Expr
%%

Expr -> Result<u64, ()>:
      Expr 'ADD' Term { Ok($1? + $3?) }
    | Term { $1 }
    ;

Term -> Result<u64, ()>:
      Term 'MUL' Factor { Ok($1? * $3?) }
    | Factor { $1 }
    ;

Factor -> Result<u64, ()>:
      'LPAR' Expr 'RPAR' { $2 }
    | 'INTEGER'
      {
          let v = $1.map_err(|_| ())?;
          parse_int($lexer.span_str(v.span()))
      }
    ;
%%

fn parse_int(s: &str) -> Result<u64, ()> {
    match s.parse::<u64>() {
        Ok(val) => Ok(val),
        Err(_) => Err(())
    }
}
```
Can you spot where we do the evaluation of addition and multiplication?

Out `main.rs`:

```rust
use std::env;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("coconut.l"); // brings the Lexer for `coconut.l` into scope.
lrpar_mod!("coconut.y"); // brings the Parser for `coconut.y` into scope.

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = &args[1]; // Create a lexer
        let lexer_def = coconut_l::lexerdef(); // Lex the input.
        let lexer = lexer_def.lexer(&input);
        let (res, errs) = coconut_y::parse(&lexer); // Parse the input.
        // Check for errors
        for e in errs {
            println!("{}", e.pp(&lexer, &coconut_y::token_epp));
        }
        // Print results
        match res {
            Some(Ok(r)) => println!("{:?}", r),
            _ => eprintln!("Unable to evaluate expression."),
        }
    } else {
        println!("Please provide at least one cli argument!")
    }
}
```

That's it. Let's add AST to our interpreter.

## Adding AST

First, let's add new file `src/ast.rs`:

```rust
#[derive(Debug)]
pub enum Node {
    Add { lhs: Box<Node>, rhs: Box<Node> },
    Mul { lhs: Box<Node>, rhs: Box<Node> },
    Number { value: u64 },
}
```
Nothing special here. 
We jsut defined our operations and the single basic type number.

Next we're going to change our Parser a bit:

```yacc
%start Nodes
%avoid_insert "INTEGER"
%%

Nodes -> Result<Vec<Node>, ()>:
    Nodes Node { flattenr($1, $2) }
  | { Ok(vec![]) }
  ;

Node -> Result<Node, ()>:
      Node 'ADD' Term {
        Ok(Node::Add{ 
          lhs: Box::new($1?), 
          rhs: Box::new($3?) 
        })
      }
    | Term { $1 }
    ;

Term -> Result<Node, ()>:
      Term 'MUL' Factor {
        Ok(Node::Mul{  
          lhs: Box::new($1?), 
          rhs: Box::new($3?) 
        })
      }
    | Factor { $1 }
    ;

Factor -> Result<Node, ()>:
      'LPAR' Node 'RPAR' { $2 }
    | 'INTEGER' { 
        match $1.map_err(|err| format!("Parsing Error: {}", err)) {
            Ok(s) => {
              let s = $lexer.span_str(s.span());
              match s.parse::<u64>() {
                  Ok(n_val) => Ok(Node::Number{ value: n_val }),
                  Err(_) => Err(())
              }
            }
            Err(_) => Err(())
        }
      }
    ;
%%
use crate::ast::Node;

/// Flatten `rhs` into `lhs`.
fn flattenr<T>(lhs: Result<Vec<T>, ()>, rhs: Result<T, ()>) -> Result<Vec<T>, ()> {
    let mut flt = lhs?;
    flt.push(rhs?);
    Ok(flt)
}
```
Note that we no longer execute the operations in our Parser, we just return enums as a vector of type `Vector<ast::Node>`.

The only thing we need to change in our `main.rs` is to add `mod ast;` to make ast module accessible
And if we run our interpreter we should get the following:

```shell
$ cargo run 2+2
[Add { lhs: Number { value: 2 }, rhs: Number { value: 2 } }]
$ cargo run 2+2*2
[Add { lhs: Number { value: 2 }, rhs: Mul { lhs: Number { value: 2 }, rhs: Number { value: 2 } } }]
```
That actually looks like an AST!.
If we format it a bit, it will make more sense:
```shell
    Add { 
      lhs: Number { value: 2 }, 
      rhs: Mul { 
        lhs: Number { value: 2 }, 
        rhs: Number { value: 2 } 
      }
    }
```
We have a tree-like structure with the correct order of operations.

That's all thanks to the grmtools Parser that does the heavy lifting here.

Looking at this tree, we can infer that we need to add 2 to a result of multiplication of 2 and 2.

And that's what we're going to do next!

## AST-time evaluation

We have our AST defined and we removed the parse-time evaluation.

Next, we're going to implement the ast-time evalution. We're going to add two functions `eval` and `eval_exp`. 

Notice that `eval_exp` is recursive, as it handles both operations and numbers.

Our `main.rs` content:

```rust
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
        let lexer_def = coconut_l::lexerdef(); // Lex the input.
        let lexer = lexer_def.lexer(&input);
        let (res, errs) = coconut_y::parse(&lexer); // Parse the input.
                                                    // Check for errors
        for e in errs {
            println!("{}", e.pp(&lexer, &coconut_y::token_epp));
        }
        // Print results
        match res {
            Some(Ok(r)) => println!("{:?}", eval(r).unwrap()),
            _ => eprintln!("Unable to evaluate expression."),
        }
    } else {
        println!("Please provide at least one cli argument!")
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
```
Let's run it.

```shell
$ cargo run 2+2*2
6
```
Looks good. 
What about paranthesie?
```shell
$ cargo run '(2+2)*3'
12
```
Also good! We preserved the operation precendece.

If you don't 100% sure how it works, there's no better teacher as your debugger. 

Try running this code and step through the evaluation of our brand new AST. It will be worth it, I promise.

## Tests

It's about time to add some tests to our interpreter.
I am not going to cover all the edge cases, but we'll add at lest a demostration of how to test our code.

You'll notice that its not going to be easy to test our code. And its not cause its rocket science, its just cause its strucutred in a non-testable way.

Let's add a few changes. 
What we want is to test string input and assert the output. So let's do just that.
We're going to extract the main login from `main` to `from_str`. The new function `from_str` will accept a string and will return result. That change will allow us to streamline our tests.

Our `main.rs`` with single test:

```rust
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
```
Now, ;ets run the tests:
```shell
$ cargo test
running 1 test
test eval_expressions ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
Now we can sleep at night knowing that we're not breaking our code as add more changes to our coconut interpreter.

Let's wrap it up.
# Summary

In this article we talked about our previous implementation of a parse-time evaluation interpreter.

We introduced the concept of AST, and compared it to Parse-tree and moved the evaluation from parse-time to ast-time. 

We talked briefly about tests, and showed how can we restructure our code to make it more testable.

This article was written for my own sake of understanding and the organisation of my thoughts as it was about knowledge sharing. 

I trust that it proved valuable!

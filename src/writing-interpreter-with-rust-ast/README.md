# Writing interpreter in Rust using grmtools
TODO: 


## Terminology

**Parse Tree** - includes all the "useless" syntactic information that humans like/need but doesn't affect compilation

**AST** - strip out that useless syntactic stuff

**Evaluator** - evaluates something (parse tree, AST, or opcodes) directly; a "compiler" converts one thing into another


## Abstrat

This article is a continuation of precvious one https://medium.com/@p3ld3v/writing-interpreter-in-rust-using-grmtools-7a6a0458b99f

Here we're going to add ast module.


## Where we ended up

coconut.l
```rust
%%
[0-9]+ "INTEGER"
\+ "ADD"
\* "MUL"
\( "LPAR"
\) "RPAR"
[\t\n ]+ ;
```

coconut.y content:

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
and out main.rs:

```
use std::env;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("coconut.l"); // brings the lexer for `coconut.l` into scope.
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

## Adding AST module

First we're going to create new file `src/ast.rs`:

```rust
#[derive(Debug)]
pub enum Opcode {
    Add { lhs: Box<Opcode>, rhs: Box<Opcode> },
    Mul { lhs: Box<Opcode>, rhs: Box<Opcode> },
    Number { value: u64 },
}
```
Nothing special here. 
We jsut defined our operations (add nad mull) and basic types (number) in rust code.

Next we're going to change our lex file a bit to return a list of Ast nodes:

```yacc
%start Opcodes
%avoid_insert "INT"
%expect-unused Unmatched "UNMATCHED"
%%

Opcodes -> Result<Vec<Opcode>, ()>:
    Opcodes Opcode { flattenr($1, $2) }
  | { Ok(vec![]) }
  ;

Opcode -> Result<Opcode, ()>:
      Opcode '+' Term {
        Ok(Opcode::Add{ 
          lhs: Box::new($1?), 
          rhs: Box::new($3?) 
        })
      }
    | Term { $1 }
    ;

Term -> Result<Opcode, ()>:
      Term '*' Factor {
        Ok(Opcode::Mul{  
          lhs: Box::new($1?), 
          rhs: Box::new($3?) 
        })
      }
    | Factor { $1 }
    ;

Factor -> Result<Opcode, ()>:
      '(' Opcode ')' { $2 }
    | 'INT' { 
        match $1.map_err(|err| format!("Parsing Error: {}", err)) {
            Ok(s) => {
              let s = $lexer.span_str(s.span());
              match s.parse::<u64>() {
                  Ok(n_val) => Ok(Opcode::Number{ value: n_val }),
                  Err(_) => Err(())
              }
            }
            Err(_) => Err(())
        }
      }
    ;

Unmatched -> ():
      "UNMATCHED" { };
%%
use crate::ast::Opcode;

/// Flatten `rhs` into `lhs`.
fn flattenr<T>(lhs: Result<Vec<T>, ()>, rhs: Result<T, ()>) -> Result<Vec<T>, ()> {
    let mut flt = lhs?;
    flt.push(rhs?);
    Ok(flt)
}
```
Note that we no longer execute the operations at the parser, we just return enum types.


The only thing we need to change in our `main.rs` is to add `mod ast;` to make ast module accessible

Our main.rs content should look something like:
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
            Some(Ok(r)) => println!("{:?}", r),
            _ => eprintln!("Unable to evaluate expression."),
        }
    } else {
        println!("Please provide at least one cli argument!")
    }
}
```
And if we run our interpreter we should get the following:

```shell
$ cargo run 2+2
[Add { lhs: Number { value: 2 }, rhs: Number { value: 2 } }]
$ cargo run 2+2*2
[Add { lhs: Number { value: 2 }, rhs: Mul { lhs: Number { value: 2 }, rhs: Number { value: 2 } } }]
```
That starts looking like an AST!
We have a tree-like structure with the correct order as well.
If we illustrate `2+2*2` as a tree, it will be:

```shell
    Add { 
      lhs: Number { value: 2 }, 
      rhs: Mul { 
        lhs: Number { value: 2 }, 
        rhs: Number { value: 2 } 
      }
    }
```
This structure tell is that we need to add 2 to a result of multiplication of 2 and 2.
And that's what we're going to do next!

## AST-time evaluation

Instead of just printing the AST to the console. 
We're going to add two functions `eval` and `eval_exp`. 

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
And if we run it:

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

Also good!

If you don't 100% sure how it works, there's no better teacher as your debugger. 
Try running this code and step through the evaluation of our brand new AST. It will be worth it, I promise.






# Summary
In this article we indtoruduced a few new concepts, such as AST....

Ref: https://github.com/softdevteam/pavel.yaiwr/commit/83512dd491b15f73369a80ef6aad7c8736ad8c4c
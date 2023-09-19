# Writing interpreter in Rust using grmtools
TODO: 


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


TODO: 

# Summary

TODO: 


[1] https://github.com/softdevteam/pavel.yaiwr/commit/83512dd491b15f73369a80ef6aad7c8736ad8c4c
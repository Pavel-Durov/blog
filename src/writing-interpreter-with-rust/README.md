# Writing interpreter in Rust using grmtools.

## Introduction

This article overviews the process of writing an interpreter with Rust programming language. We will use the `grmtools` Rust crate to help with the parsing. We will define tokens of our language and create math addition and multiplication expressions. We will also cover some basic terms and concepts related to writing an interpreter as we go. This writing is heavily based on `grmtools` [Quickstart](https://softdevteam.github.io/grmtools/master/book/quickstart.html) [1]. 


## What are grmtools?

`Grmtools` is a collection of Rust libraries for text parsing at compile and run times. We will focus on the compile-time YACC features of `grmtools` as it will provide us with the framework for parsing instead of writing one from scratch.

## What's YACC?

**YACC** (Yet Another Compiler-Compiler) is an **LALR** (Look Ahead Left-to-Right Rightmost Derivation) parser generator [2].

Don't let these acronyms scare you. **LALR** parser is just a type of **Parser**. A **Parser** is a compiler component that translates the raw source code into a meaningful structured form.

**YACC** is written in a similar notation to **BNF** (Backus–Naur Form) [3], which will resemble what we will use. The only difference is that we'll have some Rust code and types involved.


## Why YACC?

We don't have to use **YACC** or any **YACC-compatible** tools like `grmtools`, but it saves us time from defining our own Lexers and Parsers from scratch.
If you do want to implement lexing and parsing from scratch, I would suggest reading one of these books:

[Crafting Interpreters](https://www.goodreads.com/book/show/58661468-crafting-interpreters)

[Writing Interpreter in Go](https://www.goodreads.com/en/book/show/32681092-writing-an-interpreter-in-go)


## Getting started

Grmtools includes a **YACC-style** **Parser** called `lrpar` and a 
**Lexer** called `lrlex`. We will also use `cfgrammar` which we'll use to identify the **YACC** variant we will use - `YaccKind::Grmtools`.


### Terminology:

**Lexeme** - Part of the source code text

**Lexer** - Breaks the text into individual lexemes

**Parser** - Validates whether lexemes fit formal grammar

**Tokens** - Parsed **Lexemes** with the assigned type


In the next sections, we will define our **Lexer** and **Parser** in a **YACC-y** way using `grmtools` libraries in a Rust project.

## Creating new project

I will call my project **Coconut** for two main reasons: Just because I can and because I like coconuts.

Creating new cargo project:
```shell
$ cargo new coconut
```

It should have the following content:

```shell
$ tree coconut
    coconut
    ├── Cargo.toml
    └── src
        └── main.rs
```

## Setting up Cargo.toml

```toml
[package]
name = "coconut"
version = "0.0.1"
edition = "2021"

[[bin]]
doc = false
name = "coconut"

[build-dependencies]
cfgrammar = "0.13"
lrlex = "0.13.1"
lrpar = "0.13.1"

[dependencies]
cfgrammar = "0.13"
lrlex = "0.13.1"
lrpar = "0.13.1"
```

Now we have all the dependencies we need.

## Lexer 

First we need to define the vocabulary we will use in our interpreter. For that, we'll create a `coconut.l` file in our `src` directory and define our keywords as key-value pairs, where the key is a normal Regex expression and the value is the formal **Lexeme** assigned to it, separated by whitespace.

```%%
[0-9]+ "INTEGER"
\+ "ADD"
\* "MUL"
\( "LPAR"
\) "RPAR"
[\t\n ]+ ;
```

Let's examine it line-by-line:

`[0-9]+ "INTEGER"`- Any string that will match the numeric sequence `0-9` will be identified as an `INTEGER` **Lexeme**. 

`\+ "ADD"` - "+" characters will be identified as an `ADD` **Lexeme**.

`\* "MUL"` - "*" characters will be identified as a `MUL` **Lexeme**.

`\( "LPAR"` - "(" characters will be identified as an `LPAR` **Lexeme**.

`\) "RPAR"` - ")" characters will be identified as an `RPAR` **Lexeme**.

`[\t\n ]+ ;` - Any tabs, new lines and whitespaces will be replaced with empty strings. i.e. removed.

That should give you a general idea of what is possible. We can define any Regex and assign a formal lexeme to it that will be later used in the **Parser** if a text sequence is matched by the Regex pattern.


## Parser

Defining the **Parser** will involve creating `coconut.y` in our `src` directory with the following content:
```
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

The `coconut.y` grammar file has three parts, separated by the `%%` lines.

### General settings - 1st Part
This is the part where general settings are set. At a minimum, it needs the start rule `%start` to know what rule should be the starting point for the Parser.

### YACC Grammar rules - 2nd part

Our example consists of three rules: `Expr`, `Term`, and `Factor` and six productions, also known as alternatives.

Each rule can have multiple productions. Each production is separated by `|` character. Rules end with `;` a symbol.

Productions can reference other rules or **Lexemes** defined in the `coconut.l` file. If the production pattern matches, its action code is executed. 

Action code is set between `{` and `}` symbols.

The `$x` variables refer to the respective **Lexeme** in the production. 

For example:

`Term { $1 }` references the first `$1` symbol.

while 

`'Expr 'ADD' Term { Ok($1? + $3?) }` rule will reference the first `$1` and the third `$3` symbols.

### Rust code - 3rd part

Any Rust code that can be called by production action code can be defined here. We can also import other rust code from the crate or external crates.

In our example, that's where we defined the `parse_int` function.

And there we have it! We went through the `grmtools` Quickstart example and defined our **Lexer** and **Parser**. 

Next, we'll wire up our application!

## Compiling our Parser to Rust

In order to use the grammar we just defined, we need to compile it into Rust code.
For that, we will create a `build.rs` file that will provide us with such functionality.

Create `build.rs` file in the root of the project with the following content:
```
use cfgrammar::yacc::YaccKind;
use lrlex::CTLexerBuilder;

fn main() {
    CTLexerBuilder::new()
        .rust_edition(lrlex::RustEdition::Rust2021)
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .rust_edition(lrpar::RustEdition::Rust2021)
                .grammar_in_src_dir("coconut.y")
                .unwrap()
        })
        .lexer_in_src_dir("coconut.l")
        .unwrap()
        .build()
        .unwrap();
}
```

If you're unfamiliar with `build.rs` see more information in the [The Cargo Book](https://doc.rust-lang.org/cargo/reference/build-scripts.html) [4]

## Application entry point

We did the Lexing, Parsing and Compilation steps. Here we will hook all of it to our application entry point - `marin.rs`.

Our `main.rs` content:

```rust
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

In this code, we're accepting a single CLI argument as input, **Lexing** it, **Parsing** it and checking for errors.

And we're kind of done!

## Running our interpreter

Try adding two numbers:

```shell
$ cargo -q run '2+2'
4
```
> `-q` flag tells cargo to be quiet and not output verbose information.


Our interpreter also supports operation precedence and more complex math expressions:

```
$ cargo -q run '2+2*2+(2+2)'
10
```

If you're not impressed, it's OK. But there is something impressive going on here that we might have taken for granted.

If we're not careful, `1+2*3` math expression can be evaluated as: `1+2*3 = (1+2)*3 = 9`. Which is obviously wrong, at least by the conventional definition of `+` and `*` math operations. We know that `1+2*3` should be evaluated as `1+(2*3) = 7`.

We won't dive into how and why it worked; assume some magic is happening behind the scenes. 
But if you're curious, it is something to do with the fact that we just wrote an **LR** parser. Writing our grammar in this way forces the "correct" parsing with "correct" operation precedence. It essential mainly for arithmetic operations. Read more about it [here](https://tratt.net/laurie/blog/2020/which_parsing_approach.html) [5].


# Summary

We went through the steps of implementing our own interpreter using Rust programming language and `grmtools` `cfgrammar`, `lrlex` and `lrpar` libraries.
We went through the quickstart of `grmtools` [1] and added a bit more explanations of the terminology used.

Illustrated Rust project example can be found [here](https://github.com/Pavel-Durov/blog/tree/main/src/writing-interpreter-with-rust).

This writing was for my own sake of understanding and the organisation of my thoughts as it was about knowledge sharing. 


# References

[1] https://softdevteam.github.io/grmtools/master/book/quickstart.html

[2] https://www.geeksforgeeks.org/introduction-to-yacc/

[3] https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form

[4] https://doc.rust-lang.org/cargo/reference/build-scripts.html

[5] https://tratt.net/laurie/blog/2020/which_parsing_approach.html

## Introduction

In this article we will overview of the process of writing an interpreter with Rust programming language.
We will use `grmtools` crate to handle our parsing, we'll define our tokens and basic math expression evaluation.


# What's YACC?

YACC (Yet Another Compiler-Compiler) is a LALR (Look Ahead Left-to-Right Rightmost Derivation) parser generator.
But dont ler these acronyms scare you. LALR parser is just a specific type of a parser for programming languages.

Parser is based on a formal grammar definition, its that part of a compiler that tries to make sense of the source code.

YACC is written in a similar notation to Backus–Naur Form (BNF), which will be similar to what we're going to use. The only diffrence is that we'll have some Rust code and types involved.

Read more: https://www.geeksforgeeks.org/introduction-to-yacc/
Read more: https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form

# Why Yacc?

We don't have to use YACC or YACC-compartible tools like `grmtools`, but it saves us time from defining our own Lexers and Parsers from scratch.
If you do want to do implement is from scratch I would suggest reading one of these books:

https://www.goodreads.com/book/show/58661468-crafting-interpreters
https://www.goodreads.com/en/book/show/32681092-writing-an-interpreter-in-go

## Getting hands-on

We're going to beusing a tool called grmtools for our task.

Grmtools includes both a Yacc-style LR parser (lrpar) and a lex-style lexer (lrlex). 

Lexer - breaks input up into individual lexemes and the 

Parser - checks to see if the lexemes conform to a grammar. As the parser executes, it can either create a generic parse tree, or execute user-specified Rust code.

See quick start: https://softdevteam.github.io/grmtools/master/book/quickstart.html



## New cargo project

I'm going to call my project `coconut`, just cause I can.

```shell
$ cargo new coconut
```
We've just created a new project. It shoud have the following content:

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

This should have all the depenendencies we need.

## Setting up build.rs

We need to compile our grammar (.l file) and lexer (.y).
Next, we're going to craete `build.rs` for that:

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

## Defining our lexer 

Let's define the token we'll be using in our interpreter. 
For that we'll create `coconut.l` in our `src` directory:

```%%
[0-9]+ "INTEGER"
\+ "ADD"
\* "MUL"
\( "LPAR"
\) "RPAR"
[\t\n ]+ ;
```

Each line here is a set of two components - regex and the token id, separated by white space.

Examples:
`[0-9]+ "INT"`- any string that will match any numeric sequence `0-9` will be identified as "INTEGER" token. 

`[\t\n ]+ ;` - any tabs, new lines and whitesapces will be replaces with empty string.


## Defining our Grammer

Defining the parser will involve createing `coconut.y` in our `src` directory with the following content:
```
```

### Grammer Parts

The grammar is in 3 parts, separated by the %% lines.

1st Part - grammar settings, at a minimum the start rule (%start ...)

2nd Part - Yacc grammar. 

In our example, it consists of 3 rules: `Expr`, `Term`, and `Factor` and 6 productions (aka alternatives).
Each rule can have multiple productions, separated by `|` characters.


Pruduction symbols either reference other rules or lexemes (defined in `.l` file). 
If production pattern matched, its action code is executed.


The `$x` variables refer to the respective symbol in the production, numbered from 1 (i.e. $1 refers to the first symbol in the production).

3rd Part - Rust code which can be called by productions actions. We can set anything here.

## Application entry point

Our application parts are comming together but we don't have the main entrypoint yet. 
Let's defined our `main.rs` in `src` directory:

```rust
use std::env;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

// brings the lexer for `calc.l` into scope. By default the module name will be `calc_l`
lrlex_mod!("coconut.l");
// brings the parser for `calc.y` into scope. By default the module name will be `calc_y`
lrpar_mod!("coconut.y");

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = &args[1];
        // Create a lexer
        let lexer_def = coconut_l::lexerdef();
        // Lex the input.
        let lexer = lexer_def.lexer(&input);
        // Parse the input.
        let (res, errs) = coconut_y::parse(&lexer);
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

## Running our app

Try added two numbers:

```shell
$ cargo -q run '2+2'
4
```
`-q` flag tells cargo to be quiet and not output verbose debugging information.

Pretty cool I would say.

But it also support operation precedence and more complex math expressions:

```
$ cargo -q run '2+2*2+(2+2)'
10
```

If you're not implressed its ok.
the point is that if not careful, this expression can be avaluated as:

`1+2*3 = (1+2)*3 = 9` which is obviously wrong, at leasy by the conventional definition of `+` and `*` math operation. We know that `1+2*3` should be evaluated as `1+(2*3) = 7`/

We're not going to talk about how and why it worked, assume its some lind of magic behind the scenes. But it its todo with the fact that we just wrote an LR compatible parser. Writing our grammar in this odd way forces the "correct" parsing and operations precedence. It important mainly for arithmetic operations

See https://tratt.net/laurie/blog/2020/which_parsing_approach.html for more details.




# Summary

In this article we went through the steps of implementing your own interpreter using Rust programming langugage. 
We went throught the quickstart of grmtools and talked the steps in diffrent words.


This write-up was for my own sake of understanding and the organisation of my thoughts as it was about knowledge sharing. 


# References
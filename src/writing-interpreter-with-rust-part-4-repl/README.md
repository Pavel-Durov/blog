# Writing interpreter in Rust - Repl, Files and Comments (part-4)

# Abstract

We're continuing our journey of implementing our interpreter called Coconut.

In this article, we're going to implement the REPL (Read-Eval-Print Loop), re-organise the project and add file and comments support. It will be a collection of small changes that will extend the usability of our interpreter.

If you haven’t already, I recommend checking out my previous article:
[Writing an Interpreter in Rust: Bytecode and Stack-Based VM (Part 3)](https://medium.com/better-programming/writing-an-interpreter-in-rust-bytecode-and-stack-based-vm-part-3-943af4acf9e0)

# Introduction

A REPL (Read-Eval-Print Loop) allows the user to interactively execute code and see the results immediately.

Not all interpreters have REPL, but having REPL allows them to prototype fast, and test ideas and language features quickly.

I personally find REPL very handy especially when I am not 100% familiar with the language interface. 

# Project Structure

I have restructured the code a bit, but nothing fundamentally changed from the last implementation, I just moved the code form the main to separate modules inorder to have a clear separation of concern.

Our project content:

```shell
$ ls -l ./src

ast.rs - AST logic
bytecode.rs - Bytecode logic
coconut.l - Lexer logic
coconut.y - Parser logic
lib.rs - Main library crate
main.rs - Main program
parser.rs - Parsing logic
```

Our `main.rs` file is very simple:

```rust
use std::env;

use coconut::eval_str;

fn main() {
    println!("Writing Interpreter With Rust Part 4");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = &args[1];
        match eval_str(input) {
            Ok(Some(result)) => {
                println!("{}", result);
            }
            _ => eprintln!("Unable to evaluate expression."),
        }
    } else {
        println!("Please provide at least one cli argument!")
    }
}
```

The main responsibility of `main.rs` is to get the program input, call the appropriate functions of the `lib.rs` crate, and display results or errors to the console.

High-level program evaluation flow:

We get program input as CLI argument, then we call `eval_str` which will parse the string to an AST, then AST will be translated to Bytecode and then this Bytecode will be evaluated.

Let's implement the REPL.

## Implementing REPL

So far we were running our Coconut programs as CLI arguments, as:

```shell
$ cargo run '2+2'
4
```
We're about to change that!

The main component of REPL is (surprisingly) the loop, so let's start with that.

Here we go:

```rust
use std::{
    env,
    io::{stdin, stdout, Write},
};

use coconut::eval_str;

fn main() {
    println!("Writing Interpreter With Rust Part 4");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        eval(&args[1])
    } else {
        repl()
    }
}

fn repl() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(Ok(input)) => {
                if input.trim() == "exit" {
                    break;
                }
                if input.trim().is_empty() {
                    continue;
                }
                eval(&input);
            }
            _ => {}
        }
    }
}

fn eval(input: &String) {
    match eval_str(input) {
        Ok(Some(result)) => {
            println!("{}", result);
        }
        _ => eprintln!("Unable to evaluate expression."),
    }
}
```

Try it out:

```shell
$ cargo run
> 2+2
4
> 5
5
> exit
```
Works as expected, each line will be evaluated seperatly and the process will repeat until we type `exit`.

That's it, we have a REPL!

## Addding file support

So now we have two ways of executing our Coconut programs; with CLI arguments or as REPL. Let's add another way - files.

It will allow us to save programs into files and pass them to the interpreter that will evaluate the source code line by line, just as if we were typing the lines in the REPL.

Let's add file support:
```rust
use std::{
    env, fs,
    io::{stdin, stdout, Write},
};

fn main() {
    println!("Writing Interpreter With Rust Part 4");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1].ends_with(".cnt".clone()) {
            eval_file(args[1].clone())
        } else {
            eval(&args[1])
        }
    } else {
        repl()
    }
}

fn eval_file(file_name: String) {
    match fs::read_to_string(file_name) {
        Ok(content) => {
            eval(&content);
        }
        Err(_) => eprintln!("Unable to evaluate expression."),
    }
}
```
Here, before running the interpreter we check whether the CLI argument contains the extension ".cnt" (short for Coconut). If it is, we are reading the file and then evaluate it in the same fashion - line by line.

That's about it.

## Addding comments support

That might feel like a feature creep, but I thought that we could add a small feature to our interpreter anyway, as you will witness it will be a very tiny change. We're going to add comment support.

For simplicity reasons, we're going to support only single-line comments.
So if we have a file named `math.cnt` with the following content:

```
// 1+2
2+2
```
We would expect our interpreter to evaluate only the `2+2`.

Luckily, the only thing we need to do for that is to add the following line to our Lexer:

```yacc
//[^\n]*?$ ;
```

You can check the Regex yourself. Basically, it will match anything between the `//` characters and the end of the line and remove it from parsing.

Our Lexer should look something like this:

```yacc
%%
[0-9]+ "INTEGER"
\+ "ADD"
\* "MUL"
\( "LPAR"
\) "RPAR"
//[^\n]*?$ ;
[\t\n ]+ ;
```
Run it:

```rust
$ cargo run math.cnt
4
```

And don't forget about tests:
```rust
#[test]
fn comments() {
    assert_eq!(
        eval_str(&"// 2+2\n 1+1".to_string()).unwrap(),
        Some(2),
        "expected 1+1=2"
    );
    assert_eq!(
        eval_str(&"// 2+2".to_string()).unwrap(),
        None,
        "expected 1+1=2"
    );
}
```
And we're done.

# Summary

This time we didn't introduce any new concepts or components into the Coconut interpreter, we just refactored the project and added REPL, file evaluation and comment support.

REPL will give us the ability to try out our interpreter's functionality without relaunching the interpreter every time, and program files will allow us to store our Coconut programs for later evaluation. And we can add comments to our source code. What more do we need :)?

This article was written for my own sake of understanding and the organisation of my thoughts as it was about knowledge sharing.

I trust that it proved valuable!

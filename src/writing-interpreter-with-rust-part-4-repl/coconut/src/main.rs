use std::{
    env, fs,
    io::{stdin, stdout, Write},
};

use coconut::eval_str;

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
        _ => eprintln!("Unable to evaluate expression."),
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
        _ => eprintln!("Unable to evaluate expression"),
    }
}

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

#[test]
fn eval_expressions() {
    assert_eq!(
        eval_str(&"0+1*1*1".to_string()).unwrap(),
        Some(1),
        "expected 0+1*1*1"
    );
    assert_eq!(
        eval_str(&"1+1".to_string()).unwrap(),
        Some(2),
        "expected 1+1=2"
    );
    assert_eq!(
        eval_str(&"1*(1+2)".to_string()).unwrap(),
        Some(3),
        "expected 1*(1+2)=3"
    );
}

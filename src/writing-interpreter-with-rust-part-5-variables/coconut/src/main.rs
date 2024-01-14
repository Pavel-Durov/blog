use std::{
    env, fs,
    io::{stdin, stdout, Write},
};

use coconut::eval_str;

fn main() {
    println!("Writing Interpreter With Rust Part 5");
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
        Err(e) => eprintln!("Unable to evaluate expression, {}", e),
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
        Ok(None) => {}
        Err(e) => eprintln!("Unable to evaluate expression, {}", e),
    }
}

#[test]
fn test_comments() {
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

#[cfg(test)]
mod main_tests {
    use super::*;
    #[test]
    fn math_expressions() {
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
}

#[cfg(test)]
mod var_tests {
    use super::*;
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
}

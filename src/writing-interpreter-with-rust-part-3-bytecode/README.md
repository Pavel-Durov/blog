# Writing an Interpreter in Rust: Bytecode and Stack-based VM (part 3)

# Abstract

We’re continuing our journey of implementing an interpreter called Coconut in Rust!

In this article, you're diving into the concept of Bytecode and its implementation within the Coconut interpreter. We will also discuss bytecode evaluation in stack-based virtual machines and compare them to other alternatives.

If you haven’t already, I recommend checking out my previous article:
[Writing an Interpreter in Rust: AST (part 2)](https://betterprogramming.pub/writing-an-interpreter-in-rust-ast-part-2-59fd20dbc60f#ea91-3dd23bb8da31)

Let's go.

## What's Bytecode?

Bytecode is a crucial component in many interpreters and VMs (Virtual Machines). In our context, VM is a software-based environment that simulates the program execution. Our Coconut language is not running directly on the hardware, it is simulated by our VM (aka interpreter).

Here we're going to use the terms Virtual Machine and Interpreter interchangeably.

Bytecode is a low-level representation of a program's source code that is designed to be executed by the VM. It acts as an intermediary step between the high-level human-readable source code and the machine code executed by the VM.


## How is it diffrent from AST?

It might be confusing, but it's different from bytecode in its essence. AST is a structural representation of the program while Bytecode is just an instruction set that is executed by the VM.

I think it will make more sense once we'll see an actual practical example.

### Advantages of Bytecode

You might wonder why do we need so many components in our interpreter: Lexer, Parser, AST, Bytecode.
And its hard to justrify when we're hacing our toy Coconut example that all it does is adding to numbers.
But I will try :)

Here are a few advantages of bytecode:

### Portability

Bytecode can be generated once and executed on multiple target platforms. Think about it as a compilation of the source code.

### Performance and optimisations 

Interpreting and making optimisations on the Bytecode level is often faster than interpreting or compiling from the source code.

Bytecode can serve as an intermediary phase for JIT compilation. Some VMs, like the JVM (Java Virtual Machine), can compile bytecode into native machine code at runtime. JIT  can lead to significant performance gains. But it's definitely out of this article's scope.

#### Separation of Concerns

As with AST, we separated the parsing and evaluation phases. 
With Bytecode we're separating the code structural model (how the program is organized) from the execution model (how the program is run). It makes our code more modular and easier to maintain.


## Stack-based VM

Why are we talking about stacks suddenly?

There are different types of VMs, the two most common are:

Stack-based VM - stack machines use an operand stack to push and pop results to.

Stack-based VM - register machines use a number of registers to store values or pass arguments.

I chose to implement a stack-based VM cause it just simpler.
With stack-based machines, there's no need to keep track of multiple register usage when compared to Register-based VMs.

Our stack will be very simple, just a vector, an array with two functions used, `pop` and `push`.


## Defining Bytecode

Bytecode instructions encapsulate the operations and actions that the VM can perform. 
Let's define it based on the interpreter functionality we have so far:

```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add, 
    Mull,
    Push { value: u64 }, 
}
```
`Op` is short for operation.

Here's a breakdown:

Add - This instruction represents the addition operation. It will pop the top two values from the stack, perform addition, and push the result back onto the stack.

Mull - This instruction represents the multiplication operation. Similar to the Add instruction, it will pop the top two values from the stack, perform multiplication, and push the result back onto the stack.

Push -  This instruction is used to load a numeric value onto the stack.

## Implementing stack-based VM

To implement a stack-based VM, we will need to have a Stack (surprisingly enough). But we will also need to convert our AST to Bytecode since want to move away from AST-time evaluation.

And that's what we're going to do next.

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
    }
}

```
Note that `ast_to_bytecode` is recursive, since we're dealing with a tree here.

An expression such as `3+2*(2+1)` would be parsed as AST:

```shell
Add { 
        lhs: Number { value: 3 }, 
        rhs: Mul { 
            lhs: Number { value: 2 }, 
            rhs: Add { 
                lhs: Number { value: 2 }, 
                rhs: Number { value: 1 } 
        } 
    } 
}
```
And Bytcode:
```shell
Push { value: 3 }, 
Push { value: 2 }, 
Push { value: 2 }, 
Push { value: 1 }, 
Add, 
Mull, 
Add
```

Let's break down the actual evaluation of this Bytcode iterms of Add, Mul and Stack operations:

```h
                                        # stack = []
Push 3                                  # stack = [3]
Push 2                                  # stack = [3,2]
Push 2                                  # stack = [3,2,2]
Push 1                                  # stack = [3,2,2,1]
Push (Add(pop(), pop()) = 1 + 2 = 3)    # stack = [3,2,3]
Push (Mul(pop(), pop()) = 2 * 3 = 6)    # stack = [3,6]
Push (Add(pop(), pop()) = 6 + 3 = 9)    # stack = [9]
```

The result is 9. As expected!

Next, we're going to evaluate these operations one by one:

```rust
pub fn eval(ast: Vec<Node>) -> Option<u64> {
    let ops = &mut vec![];
    for a in ast {
        ast_to_bytecode(a, ops);
    }
    
    let mut stack: Vec<u64> = vec![]; // Our stack
    
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
        }
    }
    return stack.pop();
}
```

That's it. We have an interpreter that parses our syntax into AST and then generates Bytecode which is evaluated as a Stack-based VM.

Now we have basic components that will allow us to extend our interpreter functionality more easily in the future.


## Tests

We did change our interpreter internals but nothing really changed from the end user perspective, so our tests we've implemented before should be still good to go.
 
Tests are great, make more tests please!

Full source code:

`instruction.rs` - https://gist.github.com/Pavel-Durov/44dd21c246cb199ecb35a2cf31aa816f

`ast.rs` - https://gist.github.com/Pavel-Durov/44dd21c246cb199ecb35a2cf31aa816f

`main.rs` - https://gist.github.com/Pavel-Durov/85e206411b870107d7033d047fe59a8c


## Summary

We’ve implemented Bytecodes in our Coconut interpreter, and we moved away from AST-time evaluation.

We discussed briefly different VM implementations and the advantages of Stack-based VMs accompanied by practical Rust examples.

All the changes were interpreter internal only so no changes in Parsing, Lexing or Testing were needed.

This change will allow us to extend the Coconut interpreter further in the future.

Next, we will start extending the Coconut interpreter functionality and hopefully, it will be clearer why we need all these components: Lexer, Parser, AST, and Bytecode in our VM.

This article was written for my own sake of understanding and the organisation of my thoughts as it was about knowledge sharing.

I trust that it proved valuable!


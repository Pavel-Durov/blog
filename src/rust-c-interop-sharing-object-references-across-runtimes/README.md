# Navigating the Rust-C Bridge: Sharing Object References Across Runtimes

# Abstract

In this article, we will explore the complexities of Rust's interoperability with C, focusing on the challenges of passing object references between the two languages. We will explore the differences in memory management between Rust and C, examining the issues that can arise. Through practical Rust code examples, we will showcase the problems and provide potential solutions.

# Introduction

When working with Foreign Function Interface (FFI) in Rust, we often encounter the need to pass data through the language barrier. While it's straightforward to pass primitive data, such as integers that can be copied by value, things get more complicated when we deal with references and pointers.

The primary challenge arises from the various memory management mechanisms applied in different languages. C has manual memory management. It means that it requires manual instructions in code (`malloc` and `free`) to allocate and deallocate unused memory.

in Rust, memory management is scope-based; when an object goes out of scope, it's "dropped" (see [docs](https://doc.rust-lang.org/rust-by-example/trait/drop.html)). This is the fundamental way Rust reclaims memory. 

So when we pass a Rust-allocated object reference to an unmanaged C runtime, we must keep Rust's scoping in mind.

## Passing reference from Rust to C

Let's say we pass a reference to an object created in Rust to an unmanaged-memory runtime like C. What will happen if, by the time C runtime wants to act on the object, it's been already out of scope in Rust, i.e it's being freed?

In that case, what we have is UB (Undefined Behaviour) as the object will be destructed and from Rust perspective it should not be used.

Let's see an example:

```rust
use libc::{c_uint, c_void, pthread_attr_t, pthread_create, pthread_join, pthread_t, sleep};

#[derive(Debug)]
struct ThreadRoutineArgs {
    sec: c_uint,
}

impl Drop for ThreadRoutineArgs {
    fn drop(&mut self) {
        println!("Drop. {:?}", self);
        self.sec = 0;
    }
}

extern "C" fn thread_function(arg: *mut c_void) -> *mut c_void {
    let args = unsafe { (arg as *mut ThreadRoutineArgs).as_ref().unwrap() };
    println!("Thread sleeps for {} seconds", args.sec);
    unsafe {
        sleep(args.sec as c_uint);
    }
    return std::ptr::null_mut();
}

fn main() {
    let mut handle: pthread_t = 0;
    unsafe {
        let args = &ThreadRoutineArgs { sec: 3 };
        let attr: *const pthread_attr_t = std::ptr::null();
        pthread_create(
            &mut handle,
            attr,
            thread_function,
            args as *const ThreadRoutineArgs as *mut c_void,
        );
        println!("Scheduled thread with {} seconds.", args.sec);
    }

    unsafe { pthread_join(handle, std::ptr::null_mut()) };
}
```

Let's run it:

```shell
$ cargo run
Scheduled thread with 3 seconds.
Drop. ThreadRoutineArgs { sec: 3 }
Thread sleeps for 0 seconds
```

Oh! What happened there?
We can see that the `Drop` function was called before the `thread_function` ran. 
That's cause the `args` object went out of scope before the `thread_function` is called.

Let's look closely at our `main` function:

```rust
fn main() {
    ...
    unsafe {    
        let args = &ThreadRoutineArgs { sec: 3 }; // create new struct instance
        ...
    } // Here `args` Drop will be called as it goes out of scope
    ...
}
```

The root cause of such behaviour is the async nature of our code.

If it was a simple sequential execution of just calling the C function and waiting for it to complete then we would have a program that would free the `args` after the C function was called. But we have an async program, where the C function is executed after the Rust scope is ended for `args`.

We need to be careful when sharing references between different runtimes since they might handle memory in different ways.

Let's see how can we overcome this issue. 

## Boxing 

In order to overcome the issue we've just seen we will need to somehow indicate to Rust that we want to handle scoped reference differently from its default behaviour. 
Luckily we have a built-in support for that -` Box<T>`.

But first, let's talk a bit about the differences between Stack and the Heap.

### Stack vs Heap

By default, all values in Rust are allocated on the stack. Stack is a short-lived memory used to store data for function scope arguments, evaluation and return values. 

Additionally, to stack, we can also allocate memory on the Heap. Values can be allocated on the heap (boxed) by using a `Box<T>`. A box is a smart pointer to a heap-allocated value. When it goes out of scope, its destructor (drop) is called and the boxed memory is freed.

However, Box has additional methods that can help us to manage object lifetimes manually i.e. avoid calling the destructor when the boxed object goes out of scope:

`Box::into_raw` - initialises a raw pointer to the boxed object.
https://doc.rust-lang.org/std/boxed/struct.Box.html#method.into_raw

`Box:from_raw` - re-constructed box from the raw pointer.
https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw

So we can call `Box::into_raw` to a reference that we want to pass from Rust to C, and when we want to finally free it, we can call `Box:from_raw` to re-construct the `Box` object which destructor will call the destructor of T and free the allocated memory. Alternatively, we can call `drop` on it explicitly.

Example:

```rust
use libc::{c_uint, c_void, pthread_attr_t, pthread_create, pthread_join, pthread_t, sleep};

#[derive(Debug)]
struct ThreadRoutineArgs {
    sec: c_uint,
}

impl Drop for ThreadRoutineArgs {
    fn drop(&mut self) {
        println!("Drop. {:?}", self);
        self.sec = 0;
    }
}

extern "C" fn thread_function(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let args = arg as *mut ThreadRoutineArgs;
        println!("Thread sleeps for {} seconds", args.as_ref().unwrap().sec);
        sleep(args.as_ref().unwrap().sec as c_uint);
        Box::from_raw(args)
    };

    return std::ptr::null_mut();
}

fn main() {
    let mut handle: pthread_t = 0;
    unsafe {
        let args = Box::into_raw(Box::new(ThreadRoutineArgs { sec: 3 }));

        let attr: *const pthread_attr_t = std::ptr::null();
        pthread_create(
            &mut handle,
            attr,
            thread_function,
            args as *const ThreadRoutineArgs as *mut c_void,
        );
        println!(
            "Scheduled thread with {} seconds.",
            args.as_ref().unwrap().sec
        );
    }
    unsafe { pthread_join(handle, std::ptr::null_mut()) };
}
```

Run it:

```shell
$ cargo run
Scheduled thread with 3 seconds.
Thread sleeps for 3 seconds
Drop. ThreadRoutineArgs { sec: 3 }
```

That's better.
By using Boxing technique we overcame the issues we had with Rust scoping.

# Summary

In this article, we explored the complexities of sharing reference objects between Rust and C while providing practical examples to illustrate the problems and their solutions.

We briefly touched on the distinctions between stack and heap memory in Rust and how these concepts come into play.

Given the asynchronous nature of our Rust-C interop code, involving pthread_create, we navigated the deviation in memory management between the two languages. To overcome this, we used Boxing to heap-allocate the shared reference as a raw pointer using the `Box::into_raw` and `Box::from_raw` functions.

This article was primarily written to organize my thoughts and enhance my understanding of the topic, with knowledge sharing as the ultimate goal. I trust that it has proven valuable to those seeking insights into Rust-C interoperability.
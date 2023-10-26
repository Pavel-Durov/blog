# Intercepting Function Calls in C++ and Rust

## Abstract

In this article we will explore function call interception with two primary approaches: using the LD_PRELOAD environment variable and linker LD wrapper functions. 

We will demonstrate step-by-step implementation of these interception techniques, including compilation and linkage commands accompanied by practical C++ and Rust interop examples.

We will intercept the POSIX pthread_create function called from the C++ application in a client-transparent way that will not require any changes from the client side.


## Introduction

Function call interception is a technique used in computer programming to capture function calls made by a program before they reach their destination. 

This might be beneficial in different scenarios; we might want to intercept function calls in order to execute some kind of custom logic as part of the interception, for debugging purposes, dynamic patching, logging, profiling or something else.

The last time I had to do that I was working on a compiler project and we had to intercept native thread creation calls in order to execute our domain-specific logic that would support thread-local stack allocation.

## Environment

The result of compilation, linkage and runtime might be different between different operating systems or software versions.

Here is what I used to test my examples:

```shell
# OS
Debian GNU/Linux 12
# CPU
x86_64
# Clang
clang version 14.0.6
# ld
GNU ld (GNU Binutils for Debian) 2.40
# Rust
nightly-x86_64-unknown-linux-gnu (default)
rustc 1.74.0-nightly (8142a319e 2023-09-13)
```

## LD_PRELOAD 

The first interception method we'll explore will be LD_PRELOAD.

LD_PRELOAD is an environment variable. It is a linker configuration and it can be used to change the order in which ELF shared objects are loaded. Objects specified in LD_PRELOAD will be loaded before all others. 

It allows us to override ANY functionality, including the standard library. The value of LD_PRELOAD is a list where items are delimited by colons or spaces.  

There are also other methods of preloading configuration, they work in the following order:
1. The LD_PRELOAD environment variable.
2. The --preload command-line option when invoking the dynamic linker directly.
3. The /etc/ld.so.preload file.

Here we're going to focus on `LD_PRELOAD` but the same concept can be achieved through other methods.


Here's our first example:

```cpp
// file: main.cpp
#include <iostream>
#include <pthread.h>

void* thread_function(void* arg) {
    std::cout << "Thread is running." << std::endl;
    return nullptr;
}

int main() {
    pthread_t thread;
    pthread_create(&thread, nullptr, thread_function, nullptr);
    pthread_join(thread, nullptr);
    return 0;
}
```
Here we have a simple C++ program that has a single main function that creates a new thread and waits for it to finish its routine.
All the thread does is print a message to the stdout stream. Nothing fancy.

Let's define our own implementation of the `pthread_create` function with exactly the same signature:

```cpp
// file: pthread_intercept.cpp
#include <dlfcn.h>
#include <pthread.h>
#include <iostream>

typedef int (*pthread_create_t)(pthread_t*, const pthread_attr_t*, void* (*)(void*), void*);

int pthread_create(pthread_t* thread, const pthread_attr_t* attr, void* (*start_routine)(void*), void* arg) {
    pthread_create_t original_pthread_create = reinterpret_cast<pthread_create_t>(dlsym(RTLD_NEXT, "pthread_create"));
    std::cout << "Intercepted pthread_create!" << std::endl;
    return original_pthread_create(thread, attr, start_routine, arg);
}
```

Here define `pthread_create` that will be pre-loaded before the original POSIX function. 
When invoked, this function will print something to the stdout and call the original pthread_create function with the same parameters.

Compilation:

```shell
# Build shared library
$ clang++ -shared -o pthread_intercept.so pthread_intercept.cpp -fPIC
# Build main program
$ clang++ -o main main.cpp
```

And run it with `LD_PRELOAD` set:

```shell
$ LD_PRELOAD=./pthread_intercept.so ./main
Intercepted pthread_create!
Thread is running.
```
If we run the main program without `LD_PRELOAD` set we won't get the interception of course:

```shell
$ ./main
Thread is running.
```


So here we are. We intercepted the `pthread_call` call!

The only thing that is required from the client's application perspective is to define the `LD_PRELOAD` variable at runtime to make sure that our library is loaded in the right order.

Next, we're going to look at another way to intercept this function.

## LD --wrap

Let's have a look at the linker wrapper function (`ld --wrap=symbol`).

If you are not familiar with what is a linker; a linker is a program that combines several objects and archives. Essentially, if we have multiple files when we compile them together, the linker is what connects all of them. 

The linker wrapper function is a linker feature that allows us to define and use a wrapper function for any symbol we want.  That way we can override the original symbols and use them to intercept the intended calls.

Let's say we want to wrap a function called "test", the wrapper function should be called "__wrap_test" and if we wish to call the original function, it should be called "__real_test".

So there's some symbol-wrapping magic done here by the linker that handles the mapping for the new wrapper and the original symbol references.

Let's give it a go!

Our `main.cpp` content will stay the same.

But our interception code will be different since we need to define and implement the wrapper function:

Wrapper funciton in `wrap_pthread_create.cpp`:

```cpp
// file: wrap_pthread_create.cpp
#include <pthread.h>
#include <iostream>

extern "C"
{
    extern int __real_pthread_create(pthread_t *__restrict, const pthread_attr_t *__restrict,
                                     void *(*__start_routine)(void *), void *__restrict);

    int __wrap_pthread_create(pthread_t *__restrict thread, const pthread_attr_t *__restrict attr,
                              void *(*__start_routine)(void *), void *__restrict arg)
    {
        std::cout << "Intercepted pthread_create!" << std::endl;

        // Call the real pthread_create
        int result = __real_pthread_create(thread, attr, __start_routine, arg);
        std::cout << "Running code after calling the real pthread_create..." << std::endl;

        return result;
    }
}
```

Compile it:
```shell
# build shared object
$ clang++ -c -o wrap_pthread_create.o wrap_pthread_create.cpp
# build main and link the wrapper
$ clang++ -o main main.cpp wrap_pthread_create.o -Wl,--wrap=pthread_create
```
Run it:
```shell
$ main
Intercepted pthread_create!
Running code after calling the real pthread_create...
Thread is running.
```

Now we can run our compiled program without defining any environment variables or other configuration since the interception configuration was done at compile time by wrapping the original call with our own function.

### Interop example with Rust and c++

Let's see an example of how to implement the same thing we did with C++ but this time we will be intercepting `pthread_create` call from C++ in Rust.

Our main.cpp will stay as is.

Our interception Rust code will look something like this:

```rust
// file: pthread_create_intercept.rs
use std::os::raw::{c_void, c_int};

#[no_mangle]
pub extern "C" fn __wrap_pthread_create(
    _: *mut c_void,
    _: *const c_void,
    _: extern "C" fn(*mut c_void) -> *mut c_void,
    _: *mut c_void,
) -> c_int {
    eprintln!("Intercepted pthread_create!");
    return 0;
}
```

One thing to note here is that we need to compile our Rust code as `cdylib` since we want to link it it with C++.
That can be done by adding the following line to the `Cargo.toml` file:

```toml
...
[lib]
crate-type = ["cdylib"]
...
```
Official Rust linkage [docs](https://doc.rust-lang.org/beta/reference/linkage.html):
```shell
--crate-type=cdylib, #![crate_type = "cdylib"] - A dynamic system library will be produced. This is used when compiling a dynamic library to be loaded from another language. This output type will create *.so files on Linux, *.dylib files on macOS, and *.dll files on Windows.
```
Let's build it:

```shell
# build rust project
$ cargo build
# build C++ with Rust .so linkage
$ clang -o  main main.cpp ./target/debug/deps/libld_wrap_rust.so -Wl,--wrap=pthread_create
```

Note that I've manually set the path to the compiled Rust `.so` library, which would very between operating systems so might require some tweaking there.

And run it:

```
$ main
Intercepted pthread_create!
```
### Summary

We examined two distinct methods for intercepting function calls: the LD_PRELOAD environment variable and the LD wrapper functions. 

The main difference between the two is that with the LD_PRELOAD approach we require the application to change its runtime configuration, while with the wrapper function, this configuration is done at compile time. 

Both are valid approaches, the preferred application might vary based on use case specifics. I personally prefer the wrapper function approach as I find it more straightforward to understand.

We also presented practical C++ and Rust code examples, along with compilation and linkage instructions, to illustrate the implementation of these techniques. And we briefly touched on the interoperability between C++ and Rust.

There might be other ways to do it. If you know any, please reach out to me as I would love to hear about it!

This article was written for my own sake of understanding and the organisation of my thoughts as it was about knowledge sharing.

I trust that it proved valuable!
# Fucntion call interception

## Abstract

In this article, we are going to talk about the in-process function called interception.  We will provide functional examples with C and Rust and their interoperability with a focus on the POSIC pthread_create function.



## Introduction

Sometimes we might want to intercept function calls of other software.
By interception, I mean a process of stopping and catching something before it can reach its original destination.

You might wonder why would you want to do that. But it seems to be a common thing to do when your software provides some kind of services for other software. In that case you might need to do some domain-specific work in order to provide your services. 

The last time I had to do that I was working on a Programming Language Virtual Machine project and we had to intercept thread creation calls to execute our VM logic that would support thread stack allocation.

Note that the result might vary between diffrent C++ compilers and OS hosts.
I am using Intel based Ubuntu machine for these tests.

## Interception using LD_PRELOAD environment variable

LD_PRELOAD is an environment variable that can be set to load some ELF-shared objects before others. Basically, we can implement our own version of any function, including the standard library.

Intercepting function calls using LD_PRELOAD will require the program to define the LD_PRELOAD environment variable.
Using LD_PRELOAD we can force the application loader to load a shared object, over the default. In other words, we can change load priority and override functions.

Let's have a practical example.

Here we have a simple `main.cpp` file that has a single main function that creates a new thread.

```cpp
// main.cpp
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

An our thread `pthread_intercept.cpp` file:

```cpp
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
In this file we define pthread_create function with exactly the same signature as the original POSIX function. 
Then we print something to the stdout and call the original pthread_create function with the same parameters.


if we compile them:
```shell
# Build shared library
$ clang++ -shared -o pthread_intercept.so pthread_intercept.cpp -fPIC

# Build main program
$ clang++ -o main main.cpp
```

And run it with LD_PRELOAD:

```shell
$ LD_PRELOAD=./pthread_intercept.so ./main
Intercepted pthread_create!
Thread is running.
```
If we run the main program then we won't get the interception of course:

```shell
$ ./main
Thread is running.
```

So here we are. We are able to intercept the original pthread_call, do something in our code and call the original function as if we didn't intercept the call at all.
The only thing that is required from client perspective is to define the `LD_PRELOAD` variable to make sure that our library is loaded in the right order.


## LD 

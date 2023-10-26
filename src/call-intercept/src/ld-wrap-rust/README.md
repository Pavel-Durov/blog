## Intercepting pthread_create using ld --wrap flag

Use `ld` wrapper function for `pthread_create` inorder to intercept thread creation.  

Using `--wrap=pthread_create`, `pthread_create` calls will be resolved to `__wrap_pthread_create`.

## Run
```shell
$ make run
mkdir -p ./dist

clang++ -c -o ./dist/pthread_intercept.o pthread_intercept.cpp
clang++ -o ./dist/my_program main.cpp ./dist/pthread_intercept.o -Wl,--wrap=pthread_create
./dist/my_program

Intercepted pthread_create!
Running code after calling the real pthread_create...
thread_function is running.
```

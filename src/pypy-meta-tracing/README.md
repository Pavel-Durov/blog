# PyPy meta-tracing

## Abstracts

## Intro

PyPy is a Python interpreter written in Python. 
It has an automatically generated JIT compiler,
PyPy is an interesting mixture of a open source project, that sometimes had research done in it.

RPython is a grbage collected language 
RPython has no incremental compilation. You always need to compile all the parts of your VM together, which leads to infamously bad compilation times.

## PyPy features
https://www.pypy.org/features.html#stackless
 - Speed
 - Memory usage
 - 

## Tracing
Tracing just-in-time (IJT) compilation is a technique used by virtual machines (VM) to optimize the execution of a program at runtime. By recording a linear sequence of frequently executed operations, compiling them to native machine code and executing them. Opposed to traditional just-in-time (JIT) compilers that work on a per-method basis.

## Inlining [3]

Inlining is a compiler optimisation; function call site is replaced with the body of the called function. You can think of it as caching, no need to read and evaluate the function if its inlined.
Inlining occurs during compilation, without changing the source code. 

### Inlining memory impact
There is a memory impact invovlved. If a function is inlined X times, there will be X copies of the function inserted into the code. Therefore it's better to inline small functions that are called often. 

There's also something called macro expansion. Similar concept, but it occurs prior to compilation and mutates the source code that is then processed by the compiler.

## Meta-tracing
Meta-tracing doesn't make complex inlining decisions. It instead decides what to inline by precisely following what a real execution through the program is doing. Its inlining decisions are therefore very understandable and predictable, and it basically only has one heuristic based on whether the called function contains a loop or not: If the called function contains a loop, we'll never inline it, if it doesn't we always try to inline it. That predictability is actually what was the most helpful, since it makes it possible for interpreter authors to understand why the JIT did what it did and to actually influence its inlining decisions by changing the annotations in the interpreter source. It turns out that simple is better than complex. [4]




## Let's write some code

Create a simple program in RPython:

```python
import os
import sys

def entry_point(argv):
    os.write(1, bytes("Hello World!\n"))
    return 0

def target(*args):
    return entry_point, None

if __name__ == "__main__":
    entry_point(sys.argv)
```
First of all, you can run it as python directly (Python 2.7.18). 
```
$ python ./hello_world.py
Hello World!
```

Note: If we run it with Python 3+ versions, we will get some incapability errors.

### Run it with PyPy

Before running it with PyPy we first need to get PyPy. There are number of options available, see [Download page](https://www.pypy.org/download.html) [7].

We're going to download the source and the binary as following:

```bash 
VERSION=pypy2.7-v7.3.9
OS=osx64
wget -S "https://downloads.python.org/pypy/${VERSION}-src.tar.bz2" && tar -xvf "${VERSION}-src.tar.bz2"
wget -S "https://downloads.python.org/pypy/${VERSION}-${OS}.tar.bz2" && tar -xvf "${VERSION}-${OS}.tar.bz2"
```
Note that I'm downloading macOS binary. If you're running on another OS, adjust the URL accordingly.
You should have these two directies now: 

`pypy2.7-v7.3.9-osx64` - PyPy binary

`pypy2.7-v7.3.9-src` - PyPy source code

Let's run our simple RPython script via PyPy:
```
$ pypy2.7-v7.3.9-osx64/bin/pypy ./hello_world.py
Hello World!
```
Yep, looks ok. Kind of the same as with Python. Bear with me, and you will see soon why we are doing it.


### Adding tracing to RPython

What are we going to trace? That's a good question! First, let's beef up our RPython code and add a loop or something to do some work. The simplest thing I can think about for illustration purposes is calculating prime numbers, so we'll do that:

```python
import os
import sys

def prime(n):
    primes = []
    num = 0
    while num < n:
        if num > 1:
            for i in range(2, num):
                if (num % i) == 0:
                    break
            else:
                primes.append(num)
        num += 1
    return primes

def entry_point(argv):
    num = int(argv[1])
    primes = prime(num)
    os.write(1, bytes('calculated primes: \n'))
    for p in primes:
        os.write(1, bytes(str(p) + ' '))
    return 0

def target(*args):
    return entry_point, None

if __name__ == "__main__":
    entry_point(sys.argv)
```
This program accepts an argument of a number and calculates prime numbers up until that number.

Cool, I guess... Let's add some meta-tracing to it! You will not regret it.

For that, we're going to add this code:

```python
import os
import sys
from rpython.rlib.jit import JitDriver
from rpython.jit.codewriter.policy import JitPolicy


def jitpolicy(driver):
    return JitPolicy()

jitdriver = JitDriver(greens=["num", "n"], reds=["primes"])

def prime(n):
    primes = []
    num = 0
    while num < n:
        jitdriver.jit_merge_point(n=n, num=num, primes=primes)
        if num > 1:
            for i in range(2, num):
                if (num % i) == 0:
                    break
            else:
                primes.append(num)
        num += 1
    return primes

def entry_point(argv):
    num = int(argv[1])
    primes = prime(num)
    os.write(1, bytes('calculated primes: \n'))
    for p in primes:
        os.write(1, bytes(str(p) + ' '))
    os.write(1, '\n')
    return 0


def target(*args):
    return entry_point, None

if __name__ == "__main__":
    entry_point(sys.argv)
```


So we imported jit stuff from RPython and added merge point to our main loop.
We also had to define `red` and `green` variables.
Generally speaking, `red`s are the variables that are going to change, while `green`s are variables that are constant.


We have to add PyPy modules to the PYTHONPATH variable so that our program will know where to find them.
```
export PYTHONPATH=${PYTHONPATH}:${PWD}/${PYPY_VERSION}-src/
```
Run out code with jit, not
```
export PYTHONPATH=${PYTHONPATH}:${PWD}/${PYPY_VERSION}-src/
 "${PWD}/${PYPY_VERSION}-osx64/bin/pypy" "${PWD}/hello_world.py"
```

Running the translate process:

```
$ "./${PYPY_VERSION}-osx64/bin/pypy" "./${PYPY_VERSION}-src/rpython/bin/rpython" --opt=jit  "${PWD}/hello_world.py"
```


If all good, we should see some nice rainbows on the screen and at the end a summary that looks something like this:
```text
[Timer] Timings:
[Timer] annotate                       ---   4.4 s
[Timer] rtype_lltype                   ---   0.9 s
[Timer] pyjitpl_lltype                 --- 103.3 s
[Timer] backendopt_lltype              ---  32.9 s
[Timer] stackcheckinsertion_lltype     ---   0.7 s
[Timer] database_c                     ---  46.2 s
[Timer] source_c                       ---  14.4 s
[Timer] compile_c                      ---   9.7 s
[Timer] ==========================================
[Timer] Total:                         --- 212.5 s
```
We translated our RPython program to binary. It did take us some time, `212.5` seconds in fact. That's not insignificant amount, but it does the job done.


We ended up with executable called `./hello_world-c`. Run it like it's hot:
```shell
$ ./hello_world-c 10
calculated primes: 
2 3 5 7 
```

Let's compare runtimes:
```shell
$ time python hello_world.py 9999
real    0m1.607s
user    0m1.378s
sys     0m0.187s

$ time ./hello_world-c 9999
real    0m0.023s
user    0m0.010s
sys     0m0.007s
```
No surprise, pypy binary is better than CPython in crunching numbers :).

##  Not lets get those jit logs!

To get the jit logs, we need to add PYPYLOG env variable a following:
```
PYPYLOG=jit-log-opt:./log.txt ./hello_world-c 10
```
And guess what? There are no logs! What? Huge disappointment!

So... Let's play with thresholds.
Look here -> https://www.pypy.org/posts/2015/09/pypy-warmup-improvements-8349465374608676233.html

```
./pypy2.7-v7.3.9-osx64/bin/pypy --jit threshold=1,function_threshold=1 "./${PYPY_VERSION}-src/rpython/bin/rpython" --opt=jit  "${PWD}/hello_world.py"
```

We lowered the threshold for tracing loops from default of 1039 to 1, threshold for tracing functions from the start from 1619 to 1 and threshold for tracing bridges from 200 to 50. 

Bridges are "alternative paths" that JIT did not take that are being additionally traced. We believe in sane defaults, so we'll try to improve upon those numbers, but generally speaking there is no one-size fits all here.

if the tracing/backend time stays high, come and complain to us with benchmarks, we'll try to look at them





# References

  [1] https://www.pypy.org/posts/2018/09/the-first-15-years-of-pypy-3412615975376972020.html

  [2] https://en.wikipedia.org/wiki/Tracing_just-in-time_compilation

  [3] https://en.wikipedia.org/wiki/Inline_expansion

  [4] https://www.pypy.org/posts/2018/09/the-first-15-years-of-pypy-3412615975376972020.html#meta-tracing

  [5] https://speed.pypy.org/

  [6] https://rpython.readthedocs.io/en/latest/

  [7] https://www.pypy.org/download.html
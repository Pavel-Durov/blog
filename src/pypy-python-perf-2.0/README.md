# Faster Python execution with PyPy

## Abstract

In this article, we explore the usage of PyPy and RPython applications, focusing on the latest version of PyPy to date. We will compare runtime performance between PyPy and CPython and talk briefly about their implementation. 

This article serves as an update to a previous publication, [A Gentle Introduction to PyPy — Faster Python With Minimal Changes](https://medium.com/better-programming/a-gentle-introduction-to-pypy-python-performance-and-benchmarking-3d0e5609985).

## Introduction

In our previous article, [A Gentle Introduction to PyPy — Faster Python With Minimal Changes](https://medium.com/better-programming/a-gentle-introduction-to-pypy-python-performance-and-benchmarking-3d0e5609985) we delved into the world of PyPy and RPython, exploring their potential to accelerate Python execution time with minimal modifications. 
The article was inspired by the tutorial published in 2011, it was based on the translation of RPython code to binary using the PyPy toolchain. However, PyPy has evolved, offering new functionality.

## CPython, RPython and PyPy

The most widely used implementation of Python is CPython, which is likely the version running on your machine right now. In contrast, RPython is a restricted version of Python used by PyPy, and you can find more information about it in the [documentation](https://rpython.readthedocs.io/en/latest/).

PyPy, on the other hand, stands out for its impressive performance compared to CPython. It achieves this by compiling RPython to Bytecode, which is then utilized by PyPy's Just-in-time Compiler (JIT). In contrast, CPython also compiles to Bytecode, but it executes the Bytecode through the Python Virtual Machine (PVM) without any JIT.

The JIT in PyPy provides several optimizations that contribute to its speed. For example, when the JIT encounters repeatedly executed code, it can optimize its execution for efficiency.

But we're not here to talk about the internals of Python VMs and JIT optimizations, so let's move on.

## RPython example

Let's pick up the same RPython example from my previous article [A Gentle Introduction to PyPy — Faster Python With Minimal Changes](https://medium.com/better-programming/a-gentle-introduction-to-pypy-python-performance-and-benchmarking-3d0e5609985):

```python
# file: prime.py
import os
import sys


def prime(n):
    primes = []
    for num in range(0, n):
        if num > 1:
            for i in range(2, num):
                if (num % i) == 0:
                    break
            else:
                primes.append(num)
    return primes


def entry_point(argv):
    num = int(argv[1])
    primes = prime(num)
    for p in primes:
        os.write(1, bytes(str(p) + " ", 'utf-8'))
    os.write(1, bytes('\n', 'utf-8'))
    return 0


def target(*args):
    return entry_point, None


if __name__ == '__main__':
    entry_point(sys.argv)
```
As this example of RPython is a subset of CPython, we can run it with our CPython interpreter (Python 3.10.12):

```shell
$  ./prime.py 6
2 3 5 
```

And it does run just fine.

## RPython and PyPy

First, we need to get PyPy, we can find the latest versions on the official Download [page](https://www.pypy.org/download.html).

This process will depend on your machine of course. 

I am running Ubuntu Linux so I'm going to run the following:

```shell
$ wget https://downloads.python.org/pypy/pypy3.10-v7.3.12-linux64.tar.bz2
$ tar -xvf 
$ ./pypy3.10-v7.3.12-linux64/bin/pypy3.10 --version
Python 3.10.12 (af44d0b8114cb82c40a07bb9ee9c1ca8a1b3688c, Jun 15 2023, 12:39:27)
[PyPy 7.3.12 with GCC 10.2.1 20210130 (Red Hat 10.2.1-11)]
```
Looks like we're good.

> Notice that this installation will not have /rpython/translator/goal/translate.py that was used for translation in previous versions of PyPy! Check it your self!

## Running RPython with PyPy

Now that we have PyPy executable we can run it with our prime.py RPython code:
```shell
$ ./pypy3.10-v7.3.12-linux64/bin/pypy3.10 ./prime.py 6
2 3 5 
```

## Benchmarking CPython and PyPy

Here we will once again use Hyperfine to benchmark both programs, just as we did in the previous article. If you're interested in a more detailed guide on using Hyperfine, you can refer to the [Benchmarking Programs with Hyperfine](https://medium.com/@p3ld3v/benchmarking-programs-with-hyperfine-3e226f4df382) article.

For our benchmarking process, we will run both examples 100 times, preceded by 10 warm-up runs for each experiment. This approach ensures that the results are accurate and representative of the programs' actual performance.

```shell
$ hyperfine --warmup 10 --runs 20 './pypy3.10-v7.3.12-linux64/bin/pypy3.10 prime.py 1000' 'python prime.py 10
00'
Benchmark 1: ./pypy3.10-v7.3.12-linux64/bin/pypy3.10 prime.py 1000
  Time (mean ± σ):      24.1 ms ±   2.8 ms    [User: 14.0 ms, System: 10.1 ms]
  Range (min … max):    21.2 ms …  29.0 ms    20 runs
 
Benchmark 2: python prime.py 1000
  Time (mean ± σ):      15.1 ms ±   2.4 ms    [User: 13.5 ms, System: 1.6 ms]
  Range (min … max):    12.7 ms …  19.6 ms    20 runs
 
Summary
  'python prime.py 1000' ran
    1.60 ± 0.32 times faster than './pypy3.10-v7.3.12-linux64/bin/pypy3.10 prime.py 1000'
```

These results show a notable difference between PyPy and CPython runtimes when executing the same RPython code, with PyPy proving to be approximately 1.6 times faster. Though this improvement may not seem dramatic at first glance, it is not small either. Moreover, it's crucial to recognize that further optimizations can be achieved by fine-tuning the RPython code or employing different examples.

The main purpose of this demonstration aimed to showcase the capabilities and user-friendliness of PyPy. 

## Summary

In this article, we explored RPython and the process of running it with a PyPy interpreter. We also briefly touched on some fundamental differences between CPython and PyPy.

To assess PyPy's efficiency, we conducted simple benchmarks, revealing the runtime differences between PyPy and CPython when executing the same code - 1.6x speed improvement.

Furthermore, we compared the latest PyPy version to the approach described in the 2012 tutorial.

We went through RPython simple prime implementation and PyPy interpreter installation and runtime as well as covered some basic differences between CPython and PyPy. We also ran some primitive benchmarks to show the runtime differences between the two.

We also showed the difference between using PyPy with its latest versions and the way described in the 2012 tutorial.

This writing was for my own sake of understanding and organizing my thoughts as it was about knowledge sharing. I hope it was helpful. If you have questions/objections/observations/complaints, don't hesitate to reach out!

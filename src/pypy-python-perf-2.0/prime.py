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

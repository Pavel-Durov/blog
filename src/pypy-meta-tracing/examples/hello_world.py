import os
import sys
from rpython.rlib.jit import JitDriver
from rpython.jit.codewriter.policy import JitPolicy


def jitpolicy(driver):
    return JitPolicy()

def printable_location(num, n):
  return "@@@  %s_%s" % (str(num), str(n))

jitdriver = JitDriver(greens=["num", "n"], reds=["primes"], get_printable_location=printable_location)


def calculate_prime(num):
    for i in range(2, num):
        if (num % i) == 0:
            break
        else:
          return num
    return None

def prime(n):
    primes = []
    num = 1
    while num < n:
        jitdriver.jit_merge_point(n=n, num=num, primes=primes)
        prime = calculate_prime(num)
        if prime is not None:
            primes.append(prime)
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

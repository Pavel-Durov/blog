import os
import sys
# from rpython.rlib.jit import JitDriver, purefunction, hint
# from rpython.jit.codewriter.policy import JitPolicy


# def jitpolicy(driver):
#     return JitPolicy()

# jitdriver = JitDriver(greens=["pc", "tokens"], reds=["awk_heap"])
# jitdriver.jit_merge_point
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


def write(s):
    os.write(1, bytes(s, 'utf-8'))


def entry_point(argv):
    num = int(argv[1])
    primes = prime(num)
    write('calculated primes: \n')
    for p in primes:
        write(str(p) + ' ')
    write('\n')
    return 0


def target(*args):
    return entry_point, None


if __name__ == '__main__':
    entry_point(sys.argv)

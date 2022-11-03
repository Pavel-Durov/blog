import os
import sys

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
        prime = calculate_prime(num)
        if prime is not None:
            primes.append(prime)
        num += 1
    return primes

def entry_point(argv):
    num = int(argv[1])
    primes = prime(num)
    # os.write(1, bytes('calculated primes: \n'))
    print ('calculated primes: \n')
    for p in primes:
        print (str(p) + ' ')
    print ('\n')
    return 0


def target(*args):
    return entry_point, None

if __name__ == "__main__":
    entry_point(sys.argv)

# -*- coding: utf8 -*-

"""
This is the main module. It computes the equivalent resistance of a resistor grid.
"""

# a = b = tmp = 1
# n = 0
# while n < 10:
#     print a
#     a, b = b, a + b
#     n += 1


def is_prime(number):
    """Checks if the integer is a prime"""
    for i in range(2, number):
        if number % i == 0:
            return False
    return True

PRIMES = [x for x in range(2, 20) if is_prime(x)]

print PRIMES
print is_prime(10)
print is_prime(13)

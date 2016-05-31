# -*- coding: utf8 -*-

"""
This module provides utilities to handle polynomials
"""


def normalize_coefficients(coefficients):
    """
    This function removes trailing zeros
    :param coefficients:
    :return:
    """
    normalized = []
    found_non_zero = False
    for coefficient in reversed(coefficients):
        if found_non_zero or coefficient != 0:
            normalized.append(coefficient)
            found_non_zero = True
    return reversed(normalized)


class Polynomial(object):
    """
    This class represents a polynomial.
    This is mostly oriented toward polynomials with real integer coefficients but it should work
    with any coefficient.
    The coefficients are immutable
    """
    def __init__(self, coefficients):
        self.coefficients = tuple(normalize_coefficients(coefficients))

    def __str__(self):
        return str(self.coefficients)

    def __add__(self, other):
        return self.add(other)

    def __sub__(self, other):
        return self.sub(other)

    def __neg__(self):
        return self.neg()

    def __mul__(self, other):
        return self.mul(other)

    def deg(self):
        """
        Returns the degree of the polynomial.
        If the degree is negative infinity, returns -1
        :return:
        """
        return len(self.coefficients) - 1

    def add(self, other_polynomial):
        """
        Returns a new polynomial by adding the supplied polynomial to the current polynomial
        :param other_polynomial:
        :return:
        """
        new_coefficients = [0] * max(len(self.coefficients), len(other_polynomial.coefficients))
        for index, coefficient in enumerate(self.coefficients):
            new_coefficients[index] += coefficient
        for index, coefficient in enumerate(other_polynomial.coefficients):
            new_coefficients[index] += coefficient
        return Polynomial(new_coefficients)

    def neg(self):
        """
        Return the opposite polynomial (all the coefficients are the opposite)
        :return:
        """
        return Polynomial([-x for x in self.coefficients])

    def sub(self, other_polynomial):
        """
        Returns a new polynomial by subtracting the supplied polynomial from the current polynomial
        :param other_polynomial:
        :return:
        """
        return self.add(other_polynomial.neg())

    def mul(self, other_polynomial):
        """
        Returns a new polynomial by multiplying the supplied polynomial with the current polynomial
        :param other_polynomial:
        :return:
        """
        new_coefficients = [0] * (len(self.coefficients) + len(other_polynomial.coefficients))
        for self_index, self_coefficient in enumerate(self.coefficients):
            for other_index, other_coefficient in enumerate(other_polynomial.coefficients):
                new_coefficients[self_index + other_index] += self_coefficient * other_coefficient
        return Polynomial(new_coefficients)

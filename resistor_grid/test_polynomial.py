# -*- coding: utf8 -*-

"""
Unit-test for the Polynomial class
"""

import unittest

from resistor_grid.polynomial import Polynomial


class TestPolynomial(unittest.TestCase):
    """
    The TestCase for the Polynomial class
    """

    def test_constructor(self):
        """
        Test the constructor of the Polynomial class
        :return:
        """

        self.assertEqual((), Polynomial([]).coefficients)
        self.assertEqual((), Polynomial([0]).coefficients)
        self.assertEqual((1,), Polynomial([1]).coefficients)
        self.assertEqual((1,), Polynomial([1, 0]).coefficients)
        self.assertEqual((0, 1), Polynomial([0, 1]).coefficients)

    def test_deg(self):
        """
        Test the .deg method
        :return:
        """

        self.assertEqual(-1, Polynomial([]).deg())
        self.assertEqual(0, Polynomial([1]).deg())
        self.assertEqual(1, Polynomial([0, 1]).deg())

    def test_add(self):
        """
        Test the .add method
        :return:
        """

        self.assertEqual((), Polynomial([]).add(Polynomial([])).coefficients)
        self.assertEqual((1,), Polynomial([]).add(Polynomial([1])).coefficients)
        self.assertEqual((1,), Polynomial([1]).add(Polynomial([])).coefficients)
        self.assertEqual((2,), Polynomial([1]).add(Polynomial([1])).coefficients)
        self.assertEqual((1, 5), Polynomial([0, 2]).add(Polynomial([1, 3])).coefficients)
        self.assertEqual((5,), Polynomial([3, 2]).add(Polynomial([2, -2])).coefficients)

    def test_neg(self):
        """
        Test the .neg method
        :return:
        """

        self.assertEqual((), Polynomial([]).neg().coefficients)
        self.assertEqual((-1,), Polynomial([1]).neg().coefficients)
        self.assertEqual((1,), Polynomial([-1]).neg().coefficients)
        self.assertEqual((1, -2), Polynomial([-1, 2]).neg().coefficients)

    def test_mul(self):
        """
        Test the .mul method
        :return:
        """

        self.assertEqual((), Polynomial([]).mul(Polynomial([])).coefficients)
        self.assertEqual((), Polynomial([1]).mul(Polynomial([])).coefficients)
        self.assertEqual((6,), Polynomial([2]).mul(Polynomial([3])).coefficients)
        self.assertEqual((-1, 0, 1), Polynomial([1, 1]).mul(Polynomial([-1, 1])).coefficients)

    def test_div(self):
        """
        Test the .div method
        :return:
        """

        self.assertEqual((-1, 1), Polynomial([-1, 0, 1]).div(Polynomial([1, 1])).coefficients)


    def test_str(self):
        """
        Test the .__str__ method
        :return:
        """

        self.assertEqual(u"P[0]", str(Polynomial([])))
        self.assertEqual(u"P[1]", str(Polynomial([1])))
        self.assertEqual(u"P[-1]", str(Polynomial([-1])))
        self.assertEqual(u"P[x]", str(Polynomial([0, 1])))
        self.assertEqual(u"P[-x]", str(Polynomial([0, -1])))
        self.assertEqual(u"P[2x - 1]", str(Polynomial([-1, 2])))
        self.assertEqual(u"P[-2x + 1]", str(Polynomial([1, -2])))
        self.assertEqual(u"P[x^2 - 1]", str(Polynomial([-1, 0, 1])))


if __name__ == '__main__':
    unittest.main()

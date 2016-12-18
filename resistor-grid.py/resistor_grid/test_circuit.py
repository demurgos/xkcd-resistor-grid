# -*- coding: utf8 -*-

"""
Unit-test for the Circuit class
"""

import unittest

from resistor_grid.circuit import Circuit


class TestPolynomial(unittest.TestCase):
    """
    The TestCase for the Circuit class
    """

    def test_constructor(self):
        """
        Test the constructor of the Circuit class
        :return:
        """

        self.assertEqual([], Circuit(0).resistors)
        self.assertEqual([[]], Circuit(1).resistors)
        self.assertEqual([[], [0]], Circuit(2, default_value=0).resistors)
        self.assertEqual([[], [0], [0, 0]], Circuit(3, default_value=0).resistors)

    def test_swap_nodes(self):
        """
        Test the `swap_nodes` method
        :return:
        """

        circuit = Circuit(4)
        circuit.set(0, 1, 1001)
        circuit.set(0, 2, 1002)
        circuit.set(0, 3, 1003)
        circuit.set(1, 2, 1012)
        circuit.set(1, 3, 1013)
        circuit.set(2, 3, 1023)

        circuit.swap_nodes(1, 2)

        self.assertEqual(1002, circuit.get(0, 1))
        self.assertEqual(1001, circuit.get(0, 2))
        self.assertEqual(1003, circuit.get(0, 3))
        self.assertEqual(1012, circuit.get(1, 2))
        self.assertEqual(1023, circuit.get(1, 3))
        self.assertEqual(1013, circuit.get(2, 3))

        circuit = Circuit(6, default_value=0)
        circuit.set(0, 1, 1)
        circuit.set(0, 2, 1)
        circuit.set(1, 3, 1)
        circuit.set(2, 3, 1)
        circuit.set(2, 4, 1)
        circuit.set(3, 5, 1)
        circuit.set(4, 5, 1)

        circuit.swap_nodes(1, 5)

        self.assertEqual(1, circuit.get(0, 5))
        self.assertEqual(1, circuit.get(0, 2))
        self.assertEqual(1, circuit.get(5, 3))
        self.assertEqual(1, circuit.get(2, 3))
        self.assertEqual(1, circuit.get(2, 4))
        self.assertEqual(1, circuit.get(3, 1))
        self.assertEqual(1, circuit.get(4, 1))


if __name__ == '__main__':
    unittest.main()

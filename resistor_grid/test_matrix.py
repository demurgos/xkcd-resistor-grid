"""
Unit-test for the Matrix class
"""

import unittest

from resistor_grid.matrix import Matrix


class TestMatrix(unittest.TestCase):
    """
    The TestCase for the Matrix class
    """

    def test_constructor(self):
        """
        Test the constructor of the Matrix class
        :return:
        """

        # 0 lines
        self.assertEqual(Matrix([]).coefficients, ())
        # 1 line and 0 columns
        self.assertEqual(Matrix([[]]).coefficients, ((),))

        mat = Matrix([[1, 2], [3, 4]])
        self.assertEqual(mat.coefficients, ((1, 2), (3, 4)))
        self.assertEqual(mat.get_coefficient(0, 0), 1)
        self.assertEqual(mat.get_coefficient(0, 1), 2)
        self.assertEqual(mat.get_coefficient(1, 0), 3)
        self.assertEqual(mat.get_coefficient(1, 1), 4)

    def test_det(self):
        """
        Test the .compute_det method computing the determinant
        :return:
        """

        self.assertEqual(Matrix([[1]]).compute_det(), 1)
        self.assertEqual(Matrix([[1, 2], [3, 4]]).compute_det(), -2)

    def test_fill_diagonal(self):
        """
        Test the .fill_diagonal method
        :return:
        """

        filled, permut = Matrix([[0, 1], [1, 0]]).fill_diagonal()
        self.assertEqual(filled.coefficients, ((1, 0), (0, 1)))
        self.assertEqual(permut, [1, 0])


if __name__ == '__main__':
    unittest.main()

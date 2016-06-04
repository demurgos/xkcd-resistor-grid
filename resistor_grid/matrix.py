# -*- coding: utf8 -*-

"""
This module provides utilities to handle matrices.
"""

def clone_matrix(mat):
    """
    Clones a 2D array
    :param mat:
    :return:
    """
    return [[x for x in row] for row in mat]


class Matrix(object):
    """
    This class represents a 0-indexed matrix.
    """
    def __init__(self, coefficients):
        self.coefficients = tuple(tuple(x for x in row) for row in coefficients)

    def __str__(self):
        return self.pretty_print()

    def pretty_print(self, indent=8):
        """
        This returns a pretty printed version of the matrix
        :return:
        """
        formatted_lines = []
        for line in self.coefficients:
            formatted_items = []
            for item in line:
                formatted_items.append(str(item).ljust(indent, " "))
            formatted_lines.append(u"(" + ", ".join(formatted_items) + u")")
        return u"(" + u",\n ".join(formatted_lines) + u")"

    def get_coefficient(self, line, column):
        """
        Return the coefficient at line `line` and column `column`
        :param line:
        :param column:
        :return:
        """
        return self.coefficients[line][column]

    def get_size(self):
        """
        Returns the size of the matrix as the tuple (lines, columns)
        :return:
        """
        lines = len(self.coefficients)
        columns = 0 if lines == 0 else len(self.coefficients[0])
        return lines, columns

    def is_square(self):
        """
        Returns a boolean indicating whether or not the matrix is a square matrix
        :return:
        """
        lines, columns = self.get_size()
        return lines == columns

    def compute_det(self, log_progress=False):
        """
        Computes and returns the determinant of the matrix
        (Uses the Bareiss algorithm)
        :return:
        """
        if not self.is_square():
            raise Exception(u"Not a square matrix")

        mat = clone_matrix(self.coefficients)
        size = self.get_size()[0]

        for i in range(size - 1):
            for j in range(i + 1, size):
                for k in range(i + 1, size):
                    mat[j][k] = (mat[j][k] * mat[i][i]) - (mat[j][i] * mat[i][k])
                    if i > 0:
                        mat[j][k] /= mat[i - 1][i - 1]
            if log_progress:
                print i
            if i > 0:
                for j in range(size):
                    mat[j][i - 1] = 0
                    mat[i - 1][j] = 0

        return mat[size - 1][size - 1]

    def sub_matrix(self, line, column):
        """
        Returns the matrix obtained by removing the line `line` and column `column`
        :param line:
        :param column:
        :return:
        """

        mat = []
        for i, row in enumerate(self.coefficients):
            if i != line:
                new_row = []
                for j, value in enumerate(row):
                    if j != column:
                        new_row.append(value)
                mat.append(new_row)

        return Matrix(mat)

    def fill_diagonal(self):
        """
        This performs permutations to ensure that the diagonal does not contain zeros.
        :return:
        """
        if not self.is_square():
            raise Exception(u"Not a square matrix")

        mat = clone_matrix(self.coefficients)
        size = self.get_size()[0]
        permut = range(size)

        for col in range(size):
            cur_line = col
            best_line = col
            best_value = 0
            for line in range(col, size):
                cur_value = mat[line][col]
                if abs(cur_value) > best_value:
                    best_line = line
                    best_value = cur_value
            if best_value == 0:
                raise Exception(u"Singular matrix")
            permut[cur_line], permut[best_line] = permut[best_line], permut[cur_line]
            for idx in range(size):
                mat[cur_line][idx], mat[best_line][idx] = mat[best_line][idx], mat[cur_line][idx]

        return Matrix(mat), permut

    def rot_left(self):
        """
        Rotates all the coefficients to the left.
        The leftmost column becomes the rightmost column.
        :return:
        """
        width = self.get_size()[1]
        return Matrix([row[1:width] + tuple([row[0]]) for row in self.coefficients])

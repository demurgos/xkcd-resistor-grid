# -*- coding: utf8 -*-

"""
This module handles circuits.
"""

from resistor_grid.matrix import Matrix


class Circuit(object):
    """
    This class represents an electrical circuit.
    It represents the impedance between all the nodes.
    """
    def __init__(self, size, default_value=None):
        """
        Creates a new circuit of `size` nodes and resistors initialized at `default_value`
        :param size:
        :param default_value:
        """
        self.size = size
        self.resistors = None
        self.fill(default_value)

    def fill(self, fill_value):
        """
        Resets all the resistors to `default_value`
        :param fill_value:
        :return:
        """
        self.resistors = [[fill_value] * j for j in range(self.size)]

    def get(self, node1, node2):
        """
        Returns the value of the resistor between `node1` and `node2`
        :param node1:
        :param node2:
        :return:
        """
        if node1 == node2:
            raise Exception("Cannot get resistor for same node")

        return self.resistors[max(node1, node2)][min(node1, node2)]

    def set(self, node1, node2, value):
        """
        Sets the value of the resistor between `node1` and `node2`
        :param node1:
        :param node2:
        :param value:
        :return:
        """
        if node1 == node2:
            raise Exception("Cannot set resistor for same node")

        self.resistors[max(node1, node2)][min(node1, node2)] = value

    def get_matrix(self, none_value=None):
        """
        Returns the matrix associated to the circuit.
        This matrix helps to reduce the circuit
        :return:
        """
        size = 1 + (self.size * (self.size - 1)) / 2
        mat = [[none_value] * size for j in range(size)]

        # Kirchhoff's current law
        column = 0
        for i in range(self.size - 1):
            for j in range(self.size - 1 - i):
                if i > 0:
                    mat[i - 1][column] = -1
                mat[i + j][column] = 1
                column += 1

        # Kirchhoff's voltage law
        i = 1
        j = 1
        for column in range(self.size - 1, size - 1):
            j += 1
            if j >= self.size:
                i += 1
                j = i + 1
            mat[column][column] = self.get(i, j)
            if j == self.size - 1:
                for k in range(self.size - 2 - i):
                    mat[column - 1 - k][column] = -self.get(i, j)
                for k in [x * (x + 1) / 2 for x in range(self.size - 1 - i, self.size - 2)]:
                    mat[column - 1 - k][column] = self.get(i, j)
                mat[column][i - 1] = self.get(0, i)
                mat[column][self.size - 2] = -self.get(0, self.size - 1)

        # Main resistor
        mat[size - 1][0] = self.get(0, 1)  # Resistor between nodes 0 and 1
        mat[size - 1][size - 1] = 1

        return Matrix(mat)

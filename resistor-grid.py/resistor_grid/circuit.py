# -*- coding: utf8 -*-

"""
This module handles circuits.
"""
from resistor_grid.matrix import Matrix
from resistor_grid.polynomial import Polynomial

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
            raise Exception(u"Cannot get resistor for same node")

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
            raise Exception(u"Cannot set resistor for same node")

        self.resistors[max(node1, node2)][min(node1, node2)] = value

    def swap_nodes(self, node1, node2):
        """
        This method swaps the nodes node1 and node2
        :param node1:
        :param node2:
        :return:
        """
        if node1 == node2:
            return

        cur_node1 = []
        cur_node2 = []
        for i in range(self.size):
            if i != node1:
                cur_node1.append(self.get(i, node1))
            if i != node2:
                cur_node2.append(self.get(i, node2))

        for i, val in enumerate(cur_node2):
            if i >= node2:
                i += 1
            if i != node1:
                self.set(i, node1, val)

        for i, val in enumerate(cur_node1):
            if i >= node1:
                i += 1
            if i != node2:
                self.set(i, node2, val)

    def ensure_complete(self):
        """
        This method checks if all the values are known (not None).
        It raises an exception otherwise
        :return:
        """

        for i in range(self.size):
            for j in range(i):
                if self.resistors[i][j] is None:
                    raise Exception(u"Missing resistor value between {} and {}".format(j, i))

    def get_matrix(self, null_value=0, neutral_value=1):
        """
        Returns the matrix associated to the circuit.
        This matrix helps to reduce the circuit
        :return:
        """
        self.ensure_complete()

        size = 1 + (self.size * (self.size - 1)) // 2
        mat = [[null_value] * size for j in range(size)]

        # Kirchhoff's current law
        column = 0
        for block in range(self.size - 1):
            for j in range(self.size - 1 - block):
                if block > 0:
                    mat[block - 1][column] = -neutral_value
                mat[block + j][column] = neutral_value
                column += 1

        # Kirchhoff's voltage law
        block = 1
        j = 1
        for column in range(self.size - 1, size - 1):
            j += 1
            if j >= self.size:
                block += 1
                j = block + 1
            mat[column][column] = self.get(block, j)
            if j == self.size - 1:
                # print column
                for k in range(self.size - 2 - block):
                    mat[column - 1 - k][column] = -self.get(block, j)
                to_end = self.size - 2 - block
                for x in range(to_end + 1, self.size - 2):
                    k = (x * (x + 1) - to_end * (to_end + 1)) // 2
                    # print " -> ", block, to_end, k, ":", column - to_end - 1 - k
                    mat[column - to_end - 1 - k][column] = self.get(block, j)
                mat[column][block - 1] = self.get(0, block)
                mat[column][self.size - 2] = -self.get(0, self.size - 1)

        # Main resistor
        mat[size - 1][0] = self.get(0, 1)  # Resistor between nodes 0 and 1
        mat[size - 1][size - 1] = neutral_value

        return Matrix(mat)


def create_grid(width, height):
    """
    This creates a grid of resistors of size `width`×`height`
    :param width:
    :param height:
    :return:
    """
    size = width * height
    circuit = Circuit(size, default_value=Polynomial([0, 1]))
    for i in range(width):
        for j in range(height):
            if i < width - 1:
                circuit.set(j * width + i, j * width + i + 1, Polynomial([1]))
            if j < height - 1:
                circuit.set(j * width + i, (j + 1) * width + i, Polynomial([1]))
    return circuit


def create_knight_grid(width):
    """
    This creates a grid circuit were the main resistor is a knight's move away
    :param width:
    :return:
    """
    circuit = create_grid(width, width + 1)
    if width % 2 == 0:
        center = width * (width + 1) // 2 - 1
        circuit.swap_nodes(0, center - width)
        circuit.swap_nodes(1, center + width + 1)
    else:
        bellow_center = (width + 1) * (width + 1) // 2 - 1
        circuit.swap_nodes(0, bellow_center - 1)
        circuit.swap_nodes(1, bellow_center - width + 1)
    return circuit

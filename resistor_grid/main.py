# -*- coding: utf8 -*-

"""
This is the main module. It computes the equivalent resistance of a resistor grid.
"""

from resistor_grid.circuit import Circuit
from resistor_grid.polynomial import Polynomial


def generate_polygon(size):
    """
    This creates a polygonal circuit of size `size` where each resistor has R = 1 Ohm
    :param size:
    :return:
    """
    circuit = Circuit(size, Polynomial([0, 1]))
    for i in range(size):
        circuit.set(i, (i + 1) % size, Polynomial([1]))
    return circuit

CIRCUIT = generate_polygon(3)

print CIRCUIT.get_matrix(null_value=Polynomial([0]), neutral_value=Polynomial([1]))

# -*- coding: utf8 -*-

"""
This is the main module. It computes the equivalent resistance of a resistor grid.
"""

from resistor_grid.circuit import Circuit

CIRCUIT = Circuit(5)
CIRCUIT.set(0, 1, 101)
CIRCUIT.set(0, 2, 102)
CIRCUIT.set(0, 3, 103)
CIRCUIT.set(0, 4, 104)
CIRCUIT.set(1, 2, 112)
CIRCUIT.set(1, 3, 113)
CIRCUIT.set(1, 4, 114)
CIRCUIT.set(2, 3, 123)
CIRCUIT.set(2, 4, 124)
CIRCUIT.set(3, 4, 134)

print CIRCUIT.get_matrix(0)

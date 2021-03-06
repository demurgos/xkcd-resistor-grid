# -*- coding: utf8 -*-

"""
This is the main module. It computes the equivalent resistance of a resistor grid.
"""
from resistor_grid.circuit import Circuit, create_knight_grid
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


def divide_polynomials_at_infinity(pol1, pol2):
    """
    Divides pol1 by pol2 when x tends toward infinity
    :param pol1:
    :param pol2:
    :return:
    """
    deg1 = pol1.deg()
    deg2 = pol2.deg()
    if deg1 > deg2:
        raise Exception("Division leads to infinity")
    elif deg1 < deg2:
        return 0

    val1 = 0 if deg1 < 0 else pol1.coefficients[deg1]
    val2 = 0 if deg2 < 0 else pol2.coefficients[deg2]
    print(val1, u"/", val2)
    return float(val1) / float(val2)

N = 3

CIRCUIT = create_knight_grid(N)

# for x in range(N * (N + 1)):
#     for y in range(x):
#         print x, y, CIRCUIT.get(x, y)

CIRCUIT_MAT = CIRCUIT.get_matrix(null_value=Polynomial([0]), neutral_value=Polynomial([1]))
RESISTOR_MAT = CIRCUIT_MAT.sub_matrix(0, CIRCUIT_MAT.get_size()[0] - 1).rot_left()

# print CIRCUIT_MAT

CIRCUIT_VALUE = CIRCUIT_MAT.compute_det(log_progress=True)
RESISTOR_VALUE = RESISTOR_MAT.compute_det(log_progress=True)

# print RESISTOR_VALUE, u"/", CIRCUIT_VALUE

print(divide_polynomials_at_infinity(RESISTOR_VALUE, CIRCUIT_VALUE))

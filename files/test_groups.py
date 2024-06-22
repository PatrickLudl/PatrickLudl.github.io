import pytest
import numpy as np
from groups import *

def test_complex_operations():
    assert np.allclose(add_complex((3+4j), (-8+13j)), (-5+17j))
    assert np.allclose(add_complex((0-1j), (0+1j)), 0j)
    assert np.allclose(multiply_complex((3+4j), (0+0j)), 0j)
    assert np.allclose(absolute_value_complex((1+1j)), np.sqrt(2))

def test_matrix_operations():
    A = SquareMatrix([(1+1j, -8), (0, 2+3j)])
    B = SquareMatrix([(0-1j, 0), (1-5j, 2+3j)])
    assert A.are_numerically_equal(A)
    assert not A.are_numerically_equal(B)
    assert np.allclose(A.add(B).matrix, np.array([[1, -8], [1-5j, 4+6j]]))
    assert np.allclose(A.trace(), 3+4j)

@pytest.mark.parametrize("S, expected_order, expected_irreducible", [
    ([SquareMatrix([[0, 1, 0], [0, 0, 1], [1, 0, 0]]), SquareMatrix([[1, 0, 0], [0, np.exp(2*np.pi*1j/3), 0], [0, 0, np.exp(4*np.pi*1j/3)]])], 27, True),
    ([SquareMatrix([[0, 0, 1], [0, np.exp(2*np.pi*1j/4)**2, 0], [np.exp(2*np.pi*1j/4), 0, 0]]), SquareMatrix([[0, np.exp(6*np.pi*1j/4), 0], [1, 0, 0], [0, 0, np.exp(4*np.pi*1j/4)**2]])], 192, True),
    ([SquareMatrix([[1, 0], [0, -1]]), SquareMatrix([[-1, 0], [0, 1]])], 4, False)
])

def test_main_program(S, expected_order, expected_irreducible):
    ord, is_irreducible = main_program(S)
    assert ord == expected_order
    assert is_irreducible == expected_irreducible
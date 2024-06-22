import numpy as np

def add_complex(a, b):
    return np.add(a, b)

def multiply_complex(a, b):
    return np.multiply(a, b)

def absolute_value_complex(a):
    return np.abs(a)

class SquareMatrix:
    def __init__(self, matrix):
        self.matrix = np.array(matrix, dtype=np.complex_)

    def get_element(self, i, j):
        return self.matrix[i, j]

    def set_element(self, i, j, value):
        self.matrix[i, j] = value

    def are_numerically_equal(self, other):
        return np.allclose(self.matrix, other.matrix, atol=1e-9)

    def add(self, other):
        return SquareMatrix(np.add(self.matrix, other.matrix))

    def multiply(self, other):
        return SquareMatrix(np.dot(self.matrix, other.matrix))

    def trace(self):
        return np.trace(self.matrix)

def is_in(matrix, matrices):
    return any(matrix.are_numerically_equal(m) for m in matrices)

def main_program(S):
    L = S.copy()
    i = 0
    while i < len(L):
        j = 0
        while j < len(L):
            A = L[i].multiply(L[j])
            if not is_in(A, L):
                L.append(A)
            j += 1
        i += 1

    ord = len(L)
    trace_sum_squared = sum(abs(m.trace())**2 for m in L)
    is_irreducible = np.abs(trace_sum_squared - ord) <= 1e-9

    print(f"The order of the group is {ord}.")
    print(f"The representation is irreducible: {is_irreducible}")

    return ord, is_irreducible


if __name__ == "__main__":
    # Define complex numbers and matrices
    omega = np.exp(2 * np.pi * 1j / 3)
    epsilon = np.exp(4 * np.pi * 1j / 9)
    alpha = 0 - 1j / np.sqrt(3)

    g1 = SquareMatrix([[0, 1, 0], [0, 0, 1], [1, 0, 0]])
    g2 = SquareMatrix([[1, 0, 0], [0, omega, 0], [0, 0, omega**2]])
    g7 = SquareMatrix([
        [alpha, alpha, alpha],
        [alpha, alpha * omega, alpha * omega**2],
        [alpha, alpha * omega**2, alpha * omega]
    ])
    g8 = SquareMatrix([[epsilon, 0, 0], [0, epsilon, 0], [0, 0, epsilon * omega]])

    S = [g1, g2, g7, g8]

    # Measure time and compute group properties
    import time
    start_time = time.time()
    order, is_irreducible = main_program(S)
    end_time = time.time()

    print(f"Time taken for computation: {end_time - start_time:.4f} seconds.")
    # The order of the group is 648.
    # The representation is irreducible: True.
    # Time taken for computation: 1971.2705 seconds.
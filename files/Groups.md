# Checking the order of a matrix group

## Introduction

We are given a set
$$S=\{g_1, g_2,\ldots,g_n\}$$
of $m\times m$-matrices which generate a faithful $m$-dimensional representation $D$ of a finite group G. Our goals are:

* Compute the order of the group.
* Determine whether the representation $D$ defined by $S$ is reducible or irreducible.

Assuming that the order of $G$ is not too large, we can solve both problems by explicitly computing all elements of $G$ in the defining representation $D$. The number of found elements is the order of the group. $D$ is irreducible if and only if
$$\sum_{g\in G}|\mathrm{Tr}\,D(g)|^2=\mathrm{ord}\,G.$$

## Specification for computationally solving the problem

### Module complex numbers
We assume that there is a data structure which can store a real number (or a floating point approximation to it). If the programming language does not provide complex numbers:

* ComplexNumber
    + Tuple of two real numbers:
        - RealNumber Re
        - RealNumber Im
    + In the following we use the notation (Re, Im).

* ComplexNumber Add(ComplexNumber a, ComplexNumber b):
    + result = (a.Re + b.Re, a.Im + b.Im).
    + In the following we use the notation result = a + b.
    + Unit tests for implementation:
        - (3, 4) + (-8, 13) --> (-5, 17)
        - (0, -1) + (0, 1) --> (0, 0)
        - ($\sqrt{2}$, 1.5) + (1, -0.5) --> (1+$\sqrt{2}, 1.0$)
* ComplexNumber Subtract(ComplexNumber a, ComplexNumber b):
    + result = (a.Re - b.Re, a.Im - b.Im).
    + In the following we use the notation result = a - b.
* ComplexNumber Multiply(ComplexNumber a, ComplexNumber b):
    + result = (a.Re * b.Re - a.Im * b.Im, a.Re * b.Im + a.Im * b.Re).
    + In the following we use the notation result = a * b.
    + Unit tests for implementation:
        - (3, 4) * (0, 0) --> (0, 0)
        - (-1, 8) * (2, -1) --> (6, 17)
        - ($\sqrt{2}, 1$) * ($\sqrt{3}$, $\sqrt{2}$) --> ($\sqrt{6}-\sqrt{2}$, $2+\sqrt{3}$)
* RealNumber AbsoluteValue(ComplexNumber a):
    + result = $\sqrt{\mathrm{a.Re}^2 + \mathrm{a.Im}^2}$.
    + In the following we use the notation result = $|a|$.
    + Unit tests for implementation:
        - |(1,1)| --> $\sqrt{2}$
        - |(0,-1)| --> 1
        - |(3,-4)| --> 5
        - |(1/2, $-\sqrt{3}/2$)| --> 1 

### Module matrices
If programming the language does not provide matrices:

* record SquareMatrix:
    + Stores the $m^2$ elements of a complex $m\times m$-matrix.
    + Methods
        - ComplexNumber GetElement(index i, index j). In the following we use the notation value = $A_{ij}$.
        - SetElement(index i, index j, ComplexNumber value). In the following we use the notation $A_{ij}$ = value.
        - The indices i, j above denote the row and column indices of the matrix, so they can assume $m$ different values.
* Bool AreNumericallyEqual(SquareMatrix A, SquareMatrix B):
    + If for all index pairs (i,j): $|A_{ij}-B_{ij}| < 1.0\times 10^{-9}$ --> true; else: false
    + Unit tests for implementation:
        - $A = \left(\begin{matrix}(1,1) & (-8, 0)\\ (0,0) & (2,3)\end{matrix}\right)$
        - $B = \left(\begin{matrix}(0,-1) & (0, 0)\\ (1,-5) & (2,3)\end{matrix}\right)$
        - $C = \left(\begin{matrix}(1,0) & (-8, 0)\\ (1,-5) & (4,6)\end{matrix}\right)$
        - $D = \left(\begin{matrix}(-7,39) & (-16, -24)\\ (17,-7) & (-5,12)\end{matrix}\right)$
        - For all X in {A, B, C, D}: AreNumericallyEqual(X, X) --> true
        - AreNumericallyEqual(A, B) --> false
        - AreNumericallyEqual(A, C) --> false
* SquareMatrix Add(SquareMatrix A, SquareMatrix B):
    + For all index pairs (i,j):
        - $\text{result}_{ij}$ = $A_{ij}$ + $B_{ij}$
    + In the following we use the notation result = $A + B$.
    + Unit tests for implementation:
        - AreNumericallyEqual(A+B, C) --> true
        - AreNumericallyEqual(A+B, D) --> false
* SquareMatrix Multiply(SquareMatrix A, SquareMatrix B):
    + For all index pairs (i,j):
        - $\mathrm{result}_{ij}$ = $\sum_{k} A_{ik}*B_{kj}$.
        - The sum in the above equation uses the addition of complex numbers.
    + In the following we use the notation result = A * B.
    + Unit tests for implementation:
        - AreNumericallyEqual(A*B, C) --> false
        - AreNumericallyEqual(A*B, D) --> true
* ComplexNumber Trace(SquareMatrix A):
    + result = $\sum_{i} A_{ii}$.
    + In the following we use the notation result = Tr(A).
    + Unit tests for implementation:
        - Tr(A) --> (3,4)
        - Tr(B) --> (2,2)
        - Tr(C) --> (5,6)
        - Tr(D) --> (-12,51)

### Main Program

* Start of program: Given $S=\{g_1, g_2,\ldots,g_n\}$. The $g_i$ are complex $m\times m$-matrices.
* List L of known group elements: L = S.
* We now construct the whole group by multiplication of each group element with any other element. How to do this is described in the following.
* Bool IsIn(ComplexSquareMatrix A, L):
    + For each B in L:
        - If AreNumericallyEqual(A, B): return true
    + return false
    + In the following we will use the notation result = $A\in L$
* i,j = 1.
* While(i<= Number of elements of L):
    + While(j<= Number of elements of L):
        + $A = L_i * L_j$
        + If $A\not\in L$:
            - $A$ is a new group element. --> Add it to $L$: $L = L \cup \{A\}$.
        + j = j+1
    + i = i+1
* L contains all group elements.
* ord = Number of elements of L.
* Bool isIrreducible = ($|\sum_i |Tr(L_i)|^2 - \mathrm{ord}| \leq 10^{-9}$).
* Display: The order of the group is {ord}.
* Display: The representation is irreducible: {isIrreducible}.
* Unit tests for implementation:
    + $g_1 = \left(\begin{matrix}0 & 1 & 0\\ 0 & 0 & 1\\1 & 0 & 0\end{matrix}\right)$, where $1 = (1,0)$ and $0 = (0,0)$
    + $g_2 = \left(\begin{matrix}1 & 0 & 0\\ 0 & \omega & 0\\0 & 0 & \omega^2\end{matrix}\right)$, where $\omega = \exp(2\pi i/3) = (\cos(2\pi/3), \sin(2\pi/3))$
    + $S = \{g_1, g_2\}$ --> Order = 27, Irreducible: true.
    + $g_3 = \left(\begin{matrix}0 & 0 & 1\\ 0 & \eta^2 & 0\\\eta & 0 & 0\end{matrix}\right)$, where $\eta = \exp(2\pi i/4) = (\cos(2\pi/4), \sin(2\pi/4))$
    + $g_4 = \left(\begin{matrix}0 & \eta^3 & 0\\ 1 & 0 & 0\\0 & 0 & \eta^2\end{matrix}\right)$
    + $S = \{g_3, g_4\}$ --> Order = 192, Irreducible: true.
    + $g_5 = \left(\begin{matrix}1 & 0\\ 0 & -1\end{matrix}\right)$, where $-1 = (-1,0)$
    + $g_6 = \left(\begin{matrix}-1 & 0\\ 0 & 1\end{matrix}\right)$
    + $S = \{g_5, g_6\}$ --> Order = 4, Irreducible: false.

### Call of main program on an example

* $g_7 = \left(\begin{matrix}\alpha & \alpha & \alpha\\ \alpha & \alpha*\omega & \alpha*\omega^2\\\alpha & \alpha*\omega^2 & \alpha*\omega\end{matrix}\right)$ where $\alpha=-i/\sqrt{3}=(0,-1/\sqrt{3})$
* $g_8 = \left(\begin{matrix}\epsilon & 0 & 0\\ 0 & \epsilon & 0\\0 & 0 & \epsilon\omega\end{matrix}\right)$ where $\epsilon = \exp(4\pi i/9) = (\cos(4\pi/9), \sin(4\pi/9))$
* Compute the order and irreducibility property of the group generated by $S=\{g_1, g_2, g_7, g_8\}$.
* Measure the time needed for the computation and display it.
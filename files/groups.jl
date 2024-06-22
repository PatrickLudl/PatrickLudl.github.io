using Test
using LinearAlgebra
import Base: abs

# Creating matrices of complex numbers
A = [1+1im -8+0im; 0+0im 2+3im]
B = [0-1im 0+0im; 1-5im 2+3im]
C = [1+0im -8+0im; 1-5im 4+6im]

# Matrix operations
sumAB = A + B  # Matrix addition
prodAB = A * B  # Matrix multiplication
trA = tr(A)  # Trace of matrix A

function areNumericallyEqual(A, B; tol=1e-9)
    return all(abs.(A - B) .< tol)
end

@testset "Matrix and Complex Number Operations" begin
    @test A + B == C
    @test areNumericallyEqual(A + B, C)
    @test tr(A) == 3 + 4im
end



# Define a function to check if a matrix is in a list of matrices, considering numerical equality
function isInList(matrix, list; tol=1e-9)
    for item in list
        if all(abs.(matrix - item) .< tol)
            return true
        end
    end
    return false
end

# Define a function to check if a matrix is in a list of matrices, considering numerical equality
function isInList(matrix, list; tol=1e-9)
    for item in list
        if all(abs.(matrix - item) .< tol)
            return true
        end
    end
    return false
end

# Function to generate the group by multiplying each element with every other element
function generateGroup(generators)
    group = copy(generators)  # Start with the given generators
    i = 1
    while i <= length(group)
        for j in 1:length(group)
            new_element = group[i] * group[j]
            if !isInList(new_element, group)
                push!(group, new_element)
            end
        end
        i += 1
    end
    return group
end

# Function to determine if the representation is irreducible
function isIrreducible(group)
    ord = length(group)
    trace_sum = sum(abs(tr(g))^2 for g in group)
    return isapprox(trace_sum, ord, atol=1e-9)
end

@testset "Group Order and Irreducibility Tests" begin
    # Define complex roots of unity
    ω = exp(2π * im / 3)
    η = exp(2π * im / 4)

    # Define generators
    g1 = [0 1 0; 0 0 1; 1 0 0]
    g2 = [1 0 0; 0 ω 0; 0 0 ω^2]
    g3 = [0 0 1; 0 η^2 0; η 0 0]
    g4 = [0 η^3 0; 1 0 0; 0 0 η^2]
    g5 = [1 0; 0 -1]
    g6 = [-1 0; 0 1]

    # Test groups
    group1 = generateGroup([g1, g2])
    group2 = generateGroup([g3, g4])
    group3 = generateGroup([g5, g6])

    @test length(group1) == 27
    @test isIrreducible(group1) == true

    @test length(group2) == 192
    @test isIrreducible(group2) == true

    @test length(group3) == 4
    @test isIrreducible(group3) == false
end


# Use the previously defined functions
function main()
    # Define the complex roots of unity and other constants
    α = -im/sqrt(3)
    ω = exp(2π * im / 3)
    ε = exp(4π * im / 9)

    # Define the generators g7 and g8
    g7 = [α α α; α α*ω α*ω^2; α α*ω^2 α*ω]
    g8 = [ε 0 0; 0 ε 0; 0 0 ε*ω]

    # Generate the group
    start_time = time()
    group = generateGroup([g7, g8])
    elapsed_time = time() - start_time

    # Determine group properties
    group_order = length(group)
    irreducible = isIrreducible(group)

    # Display results
    println("The order of the group is: $group_order")
    println("The representation is irreducible: $irreducible")
    println("Time needed for the computation: $elapsed_time seconds")
end

main()

#=
Test Summary:                        | Pass  Total  Time
Matrix and Complex Number Operations |    3      3  0.2s
Test Summary:                        | Pass  Total  Time
Group Order and Irreducibility Tests |    6      6  1.6s
The order of the group is: 648
The representation is irreducible: true
Time needed for the computation: 14.103000164031982 seconds
=#
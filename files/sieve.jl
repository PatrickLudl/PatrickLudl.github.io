function flip!(sieve_list::BitVector, n::Int)
    if n <= length(sieve_list)
        sieve_list[n] = !sieve_list[n]
    end
end

function sieve_of_atkin(N::Int)
    if N <= 6
        println("The number must be greater than 6.")
        return []
    end

    list_of_primes = [2, 3, 5]
    sieve_list = falses(N)

    limit = floor(Int, sqrt(N))

    for x in 1:limit
        for y in 1:limit
            n = 4 * x^2 + y^2
            if n <= N && n % 60 in (1, 13, 17, 29, 37, 41, 49, 53)
                flip!(sieve_list, n)
            end

            n = 3 * x^2 + y^2
            if n <= N && n % 60 in (7, 19, 31, 43)
                flip!(sieve_list, n)
            end

            n = 3 * x^2 - y^2
            if x > y && n <= N && n % 60 in (11, 23, 47, 59)
                flip!(sieve_list, n)
            end
        end
    end

    for k in 7:N
        if sieve_list[k]
            push!(list_of_primes, k)
            for m in 1:floor(Int, N / k^2)
                sieve_list[m * k^2] = false
            end
        end
    end

    return list_of_primes
end

# Example usage with timing test:
N = 10^9
@time primes = sieve_of_atkin(N)
println("Number of primes found: ", length(primes))

# Unit tests
using Test

@test length(primes) == 50847534

# Known prime numbers up to 100
known_primes_up_to_100 = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
@test sieve_of_atkin(100) == known_primes_up_to_100

# Highest prime number less than 10^6
highest_prime_less_than_10_6 = 999983
@test maximum(sieve_of_atkin(1000000)) == highest_prime_less_than_10_6

println("All tests passed.")
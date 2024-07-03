# Sieve of Atkin

The Sieve of Atkin and Bernstein is a modern algorithm to compute the prime numbers smaller than N. In the description below, we follow the Wikipedia Article [SieveOfAtkin](https://en.wikipedia.org/wiki/Sieve_of_Atkin).
We describe the program flow both as text, and graphically.

## Program

### Function Flip(list<bool> SieveList, int n)
* If n <= length(SieveList): Flip the entry: n-th entry of SieveList = not(n-th entry of SieveList).

### Main Program
* Get integer number N > 6 as input.
* Initialize ListOfPrimes = [2, 3, 5].
* Create a list SieveList of N bools, all entries false.
* For x in {1,...,$\mathrm{floor}(\sqrt{N})$}
    + For y in {1,...,$\mathrm{floor}(\sqrt{N)}$}
        - $n = 4 x^2 + y^2$
            * If n mod 60 in {1, 13, 17, 29, 37, 41, 49, 53}: Flip(SieveList, n).
        - $n = 3 x^2 + y^2$
            * If n mod 60 in {7, 19, 31, 43}: Flip(SieveList, n).
        - $n = 3 x^2 - y^2$
            * If x > y and n mod 60 in {11, 23, 47, or 59}: Flip(SieveList, n).
* While there are entries true in the SieveList:
    + Determine k such that: The k-th entry of SieveList is the first with entry true:
        - Append k to ListOfPrimes.
        - Set k-th entry of SieveList to false (we do not want to check it again).
        - Set m*k^2-th entry of SieveList to false for all multiples m*k^2 of k^2 (m = 1,2,...,floor(N/k^2)).
* Return ListOfPrimes.

### Main Program as mermaid.js graph

```mermaid.js
graph TD
    A["Start"] --> B["Get integer N > 6 as input"]
    B --> C{"N > 6?"}
    C -- "No" --> D["Print 'The number must be greater than 6.'"]
    C -- "Yes" --> E["Initialize ListOfPrimes = [2, 3, 5]"]
    E --> F["Create a list SieveList of N bools, all entries false"]
    F --> G["For x in {1,...,floor(sqrt(N))}"]
    G --> H["For y in {1,...,floor(sqrt(N))}"]
    H --> I["n = 4 * x^2 + y^2"]
    I --> J{"n % 60 in {1, 13, 17, 29, 37, 41, 49, 53}?"}
    J -- "Yes" --> K["Flip(SieveList, n)"]
    J -- "No" --> L["n = 3 * x^2 + y^2"]
    K --> L
    L --> M{"n % 60 in {7, 19, 31, 43}?"}
    M -- "Yes" --> N["Flip(SieveList, n)"]
    M -- "No" --> O["n = 3 * x^2 - y^2"]
    N --> O
    O --> P{"x > y and n % 60 in {11, 23, 47, 59}?"}
    P -- "Yes" --> Q["Flip(SieveList, n)"]
    P -- "No" --> R["End of y loop"]
    Q --> R
    R --> H
    G --> S["End of x loop"]
    S --> T["While there are entries true in the SieveList"]
    T --> U["Determine k such that k-th entry of SieveList is the first with entry true"]
    U --> V["Append k to ListOfPrimes"]
    V --> W["Set k-th entry of SieveList to false"]
    W --> X["Set m*k^2-th entry of SieveList to false for all multiples m*k^2 of k^2"]
    X --> Y["End of while loop"]
    Y --> Z["Return ListOfPrimes"]
    Z --> AA["End"]
```
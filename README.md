# divisors
The Rust program that finds all the divisors of a given number in parallel mode!

This program splits the work of finding divisors across multiple threads using Rust's thread module. The number of threads used is determined by the num_threads variable, which can be adjusted to optimize performance.

The program divides the input number n into num_threads chunks of roughly equal size, and spawns a thread to process each chunk. Each thread computes the divisors of its chunk using a for loop, and returns the result as a Vec of divisors.

After all the threads have completed, the program collects the results by joining the threads and merging their Vecs of divisors into a single Vec. This Vec is then sorted and deduplicated to remove any duplicates, and printed to the console using Rust's println! macro.

Note that the order of the divisors in the output may be different from the sequential version of the program due to the parallel nature of the computation.

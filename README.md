Welcome to ParSage, my term project for ICCS311: Functional and Parallel Programming. ParSage an attempt to parallelize a (very tiny) subset of SageMath functions in Rust, utilizing parallelism techniques covered in class. The main focus is number theory, such as prime numbers, primitive roots, RSA, etc. Essentially, things that will require some searching, where we can divide the search space and perform searching in parallel.

The running time is reasonably fast for “standard” use but some functions take some time for large inputs (> 10^8). For example, the par_prime_factors functions takes over a minute for n that is a product of primes over 1 million. All in all, no where near as good as SageMath, although everything is parallelized (except for factor).

To test out the functions, use main.rs and access them via the already imported modules. An example is provided on the main.rs file. 

use rayon::iter::*;
use crate::primes;


fn multiply_rec(p1: u64, p2: u64) -> f64 {
    (1.0 - 1.0/(p1 as f64)) * (1.0 - 1.0/(p2 as f64))
}
pub fn euler_phi(n: u64) -> u64 {
    let factors = primes::par_prime_factors(n);
    n * factors.par_iter()
        .reduce(|| &1u64, |&a,&b| &(multiply_rec(a, b) as u64))
}
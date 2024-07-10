use rayon::prelude::*;
use crate::primes;


fn multiply_rec(p1: f64, p2: f64) -> f64 {
    p1 * (1.0 - 1.0/p2)
}
pub fn euler_phi(n: u64) -> u64 {
    let factors = primes::par_prime_factors(n);
    let prod = factors.par_iter()
        .fold(|| 1.0, |a, &e| a*(1.0 - 1.0/(e as f64)))
        .reduce(|| 1.0, |a, b| a * b);
    (n as f64 * prod) as u64
}
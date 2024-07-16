use rayon::iter::*;
use rayon::prelude::*;

pub fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        n if n % 2 == 0 || n % 3 == 0 => false,
        _ => {
            let limit = (n as f64).sqrt() as u64;
            (5..=limit).into_par_iter()
                .filter(|i| i % 6 == 5 || i % 6 == 1)
                .all(|i| n % i != 0 && n % (i + 2) != 0)
        }
    }
}

// computes pi(x), the number of primes less than or equal to n
pub fn par_pi(n: u64) -> u64 {
    (2..=n)
        .into_par_iter()
        .filter(|&i| is_prime(i))
        .count() as u64
}

/// Finds the unique prime factors of n
pub fn par_prime_factors(n: u64) -> Vec<u64> {
    (2..=n).into_par_iter()
        .filter(|&num| n % num == 0 && is_prime(num))
        .collect()
}
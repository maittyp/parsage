use rayon::prelude::*;
use crate::primes::par_prime_factors;
use crate::utility::*;

fn multiply_rec(p1: f64, p2: f64) -> f64 {
    p1 * (1.0 - 1.0/p2)
}

// computes phi(n), the number of numbers between 1 and n that are coprime to n
pub fn euler_phi(n: u64) -> u64 {
    let factors = par_prime_factors(n);
    let prod = factors.par_iter()
        .fold(|| 1.0, |a, &e| a*(1.0 - 1.0/(e as f64)))
        .reduce(|| 1.0, |a, b| a * b);
    (n as f64 * prod) as u64
}

// computes the order of a modulo n
fn ord_mod_n(a: u64, n: u64) -> u64 {
    if a%n == 1 {
        return 0
    }
    let ord = (1..n).into_par_iter()
        .find_first(|&num| pow_mod(a, num, n) == 1);
    ord.unwrap_or_else(|| u64::MAX)
}

// checks if a is a primitive root of n
pub fn is_prim_root(a: u64, n: u64) -> bool {
    ord_mod_n(a, n) == euler_phi(n)
}

// checks if there is a primitive root modulo n, utilizing a theorem from number theory
fn has_prim_root(n: u64) -> bool {
    if n%2 == 0 {
        if par_prime_factors(n).len() == 1 { n==2 || n==4 }
        else { par_prime_factors(n/2).len() == 1 }
    }
    else {
        n==1 || n==2 || n==4 || par_prime_factors(n).len() == 1
    }
}

// finds the smallest primitive root modulo n, if there is one.
pub fn prim_root(n: u64) -> u64 {
    if n==1 {return 0}
    if n==2 {return 1}
    if n==4 {return 3}
    if has_prim_root(n) {
        (1..n).into_par_iter()
            .find_first(|&num| is_prim_root(num, n))
            .unwrap()
    }
    else {
        panic!("There is no primitive root modulo {}", n)
    }
}

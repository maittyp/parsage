use rand::Rng;
use crate::utility::*;
use rayon::iter::*;
use rayon::prelude::*;

/// returns (encrypted(x), public key)
pub fn encrypt(p: u64, q: u64, x: u64) -> (u64, (u64, u64)) {
    let n = p*q;
    let phi = (p-1)*(q-1);
    let coprimes = (1..phi).into_par_iter()
        .filter(|&num| gcd(phi as i64, num as i64)==1)
        .collect::<Vec<u64>>();
    let mut rng = rand::thread_rng();
    let range = 0..coprimes.len();
    let e = coprimes[rng.gen_range(range)];
    (pow_mod(x, e, n), (n,e))
}

/// decrypts m, given a public key
pub fn decrypt(m: u64, pub_key: (u64, u64)) -> u64 {
    let (n, e) = pub_key;
    let phi = euler_phi(n);
    let d = inverse_mod(e as i64, phi as i64);
    pow_mod(m, d as u64, n)
}

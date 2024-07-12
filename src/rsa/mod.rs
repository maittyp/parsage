use rand::Rng;
use crate::utility::*;
use rayon::iter::*;
use rayon::prelude::*;

// returns (encrypt(x), public key)
pub fn encrypt(p: u64, q: u64, x: u64) -> (u64, (u64, u64)) {
    let n = p*q;
    let phi = (p-1)*(q-1);
    let coprimes = (1..phi).into_par_iter()
        .filter(|&num| gcd(phi as i64, num as i64)==1)
        .collect::<Vec<u64>>();
    let mut rng = rand::thread_rng();
    let range = 0..euler_phi(phi);
    let e = coprimes[rng.gen_range(range) as usize];
    (pow_mod(x, e, n), (n,e))
}

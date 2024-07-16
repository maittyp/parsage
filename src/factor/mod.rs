use chashmap::CHashMap;
use rayon::prelude::*;
use crate::primes::is_prime;

/// Function to completely factorize an integer
pub fn factor(n: u64) -> String {
    let mut n = n;
    let factors: CHashMap<u64, u64> = CHashMap::new();

    // Handle the factor 2
    while n % 2 == 0 {
        factors.upsert(2, || 1, |v| *v += 1);
        n /= 2;
    }

    let upper_bound = (n as f64).sqrt() as u64 + 1;

    // Parallel processing of potential factors
    (3..=upper_bound)
        .into_par_iter()
        .filter(|&i| i % 2 != 0) // Only consider odd numbers
        .for_each(|i| {
            let mut count = 0;
            let mut value = n;
            while value % i == 0 {
                count += 1;
                value /= i;
            }
            if count > 0 {
                factors.upsert(i, || count, |v| *v += count);
            }
        });

    if n > 1 {
        factors.upsert(n, || 1, |v| *v += 1);
    }

    format_factors(factors)
}

/// Format the factors as a string
fn format_factors(factors: CHashMap<u64, u64>) -> String {
    let mut result = String::new();
    let mut sorted_factors: Vec<(u64, u64)> = factors.into_iter()
        .filter(|(x,y)| is_prime(*x))
        .collect();

    sorted_factors.sort_by_key(|&(factor, _)| factor);

    for (factor, count) in sorted_factors {
        if !result.is_empty() {
            result.push('*');
        }
        if count > 1 {
            result.push_str(&format!("{}^{}", factor, count));
        } else {
            result.push_str(&factor.to_string());
        }
    }
    result
}

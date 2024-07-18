use std::collections::BTreeMap;

pub fn factor(mut number: u128) -> String {
    let mut prime_factors: BTreeMap<u128, u128> = BTreeMap::new();
    let mut freq:u128 = 0;
    while number&1 == 0 {
        number>>=1;
        freq+=1;
    }
    if freq > 0 { prime_factors.insert(2, freq);}
    let mut i = 3;
    while i*i <= number {
        if number%i==0 {
            freq = 0;
            while number%i==0 {
                number/=i;
                freq+=1;
            }
            prime_factors.insert(i, freq);
        }
        i+=2;
    }
    if number > 1 {prime_factors.insert(number, 1);}

    format_factors(prime_factors)
}

/// Format the factors as a string
fn format_factors(factors: BTreeMap<u128, u128>) -> String {
    let mut result = String::new();

    for (factor, count) in factors {
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

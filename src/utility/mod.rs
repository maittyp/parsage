// computes the greatest common divisor of a and b, using the Euclidean algorithm
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// computes a^m (mod n)
pub fn pow_mod(a: u64, m: u64, n: u64) -> u64 {
    fn helper(a: u64, m: u64, n:u64, ans: u64) -> u64 {
        if m == 0 { ans }
        else {helper(a,m-1, n,ans*a % n)}
    }
    helper(a, m, n, 1)
}

// computes x,y,g such that ax + by = g (extended Euclidean algorithm)
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        (gcd, y1, x1 - (a / b) * y1)
    }
}

// computes a^-1 such that a*a^-1 = 1 (mod n)
pub fn inverse_mod(a: i64, n: i64) -> i64 {
    if gcd(n, a) != 1 {
        panic!("Inverse of {} modulo {} does not exist", a, n)
    }
    else {
        let (_x, y, _g) = extended_gcd(a, n);
        y
    }
}
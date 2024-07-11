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

pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        (gcd, y1, x1 - (a / b) * y1)
    }
}
pub fn inverse_mod(a: i64, n: i64) -> i64 {
    if gcd(n, a) != 1 {
        panic!("Inverse of {} modulo {} does not exist", a, n)
    }
    else {
        let (x, _y, _g) = extended_gcd(a, n);
        x
    }
}
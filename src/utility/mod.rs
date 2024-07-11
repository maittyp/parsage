pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn pow_mod(a: u64, m: u64, n: u64) -> u64 {
    fn helper(a: u64, m: u64, n:u64, ans: u64) -> u64 {
        if m == 0 { ans }
        else {helper(a,m-1, n,ans*a % n)}
    }
    helper(a, m, n, 1)
}
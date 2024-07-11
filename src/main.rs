mod primes;
mod prim_roots;
mod gcd;

use std::time::Instant;



fn main() {
    let start_seq = Instant::now();

    // println!("{:?}", prim_roots::prim_root(97));

    let elapsed_seq = start_seq.elapsed();

    println!("Time taken: {:?}", elapsed_seq);

}

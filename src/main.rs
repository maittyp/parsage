mod primes;
mod prim_roots;
mod utility;
mod rsa;
mod factor;

use std::time::Instant;


fn main() {
    let start_seq = Instant::now();

    println!("{:?}", factor::factor(12525));

    let elapsed_seq = start_seq.elapsed();

    println!("Time taken: {:?}", elapsed_seq);

}

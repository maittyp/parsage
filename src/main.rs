mod primes;
mod prim_roots;
mod utility;
mod rsa;

use std::time::Instant;



fn main() {
    let start_seq = Instant::now();

    println!("{:?}", rsa::encrypt(17, 19, 23));

    let elapsed_seq = start_seq.elapsed();

    println!("Time taken: {:?}", elapsed_seq);

}

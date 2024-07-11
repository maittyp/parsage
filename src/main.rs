mod primes;
mod prim_roots;
mod utility;

use std::time::Instant;



fn main() {
    let start_seq = Instant::now();

    println!("{:?}", utility::inverse_mod(4, 6));

    let elapsed_seq = start_seq.elapsed();

    println!("Time taken: {:?}", elapsed_seq);

}

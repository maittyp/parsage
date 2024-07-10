mod primes;
mod prim_roots;

use std::time::Instant;



fn main() {
    let start_seq = Instant::now();

    println!("{:?}", prim_roots::euler_phi(100));

    let elapsed_seq = start_seq.elapsed();

    println!("Time taken: {:?}", elapsed_seq);

}

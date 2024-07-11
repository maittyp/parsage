mod primes;
mod prim_roots;
mod utility;

use std::time::Instant;



fn main() {
    let start_seq = Instant::now();

    println!("{:?}", prim_roots::prim_root(47));

    let elapsed_seq = start_seq.elapsed();

    println!("Time taken: {:?}", elapsed_seq);

}

mod primes;
mod prim_roots;
mod utility;
mod rsa;

use std::time::Instant;



fn main() {
    let start_seq = Instant::now();

    let (encrypted, public_key) = rsa::encrypt(17, 19, 24);

    println!("{:?}", rsa::decrypt(encrypted, public_key));

    let elapsed_seq = start_seq.elapsed();

    println!("Time taken: {:?}", elapsed_seq);

}

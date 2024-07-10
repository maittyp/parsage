mod primes;
use std::time::Instant;



fn main() {
    let start_seq = Instant::now();

    println!("{:?}", primes::par_pi(10000000));

    let elapsed_seq = start_seq.elapsed();

    println!("Time taken: {:?}", elapsed_seq);

}

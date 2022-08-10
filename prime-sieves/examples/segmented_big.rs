use prime_sieves::segmented::sieve_segmented;

fn main() {
    let vec = sieve_segmented(2000000000);
    println!("{}", vec.primes_result.len());
}

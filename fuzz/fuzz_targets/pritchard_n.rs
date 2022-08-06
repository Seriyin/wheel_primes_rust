#![no_main]
use libfuzzer_sys::fuzz_target;
use types::ConstrainedUSize;
use primal_sieve::Primes;
use prime_sieves::segmented;

fuzz_target!(|c: ConstrainedUSize| {
let num = c.num;
let vec_expected: Vec<usize> = Primes::all().take(num).collect();
{
    let vec_actual: Vec<usize> = segmented::n_primes_segmented(num).primes_result;
    assert_eq!(vec_actual[..], vec_expected[..])    
}
});

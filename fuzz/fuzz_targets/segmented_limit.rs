#![no_main]
use libfuzzer_sys::fuzz_target;
use types::ConstrainedUSize;
use primal_sieve::Sieve;
use prime_sieves::segmented;

fuzz_target!(|c: ConstrainedUSize| {
    let num = c.num;
    let vec_expected: Vec<usize> = Sieve::new(num).primes_from(0).take_while(|&n| n <= num).collect();

    {
        let vec_actual: Vec<usize> = segmented::sieve_segmented(num).primes_result;
        assert_eq!(vec_actual[..], vec_expected[..])    
    }

});

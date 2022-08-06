#![no_main]
use libfuzzer_sys::fuzz_target;
use types::ConstrainedN;
use primal_sieve::Primes;
use prime_sieves::pritchard;

fuzz_target!(|c: ConstrainedN| {
    let num = c.num;
    let vec_expected: Vec<usize> = Primes::all().take(num).collect();
    {
        let vec_actual: Vec<usize> = pritchard::n_primes_pritchard(num);
        assert_eq!(vec_actual[..], vec_expected[..])    
    }


});

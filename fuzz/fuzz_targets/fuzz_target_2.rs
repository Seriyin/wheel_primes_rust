#![no_main]
use libfuzzer_sys::fuzz_target;
use types::ConstrainedN;
use primal_sieve::Primes;
use prime_sieves::{pritchard, segmented};

fuzz_target!(|c: ConstrainedN| {
    let num = c.num;
    let vec_expected: Vec<usize> = Primes::all().take(num).collect();
    assert_pritchard(num, &vec_expected);
    assert_segmented(num, &vec_expected);
});


fn assert_pritchard(num: usize, vec_expected: &Vec<usize>) {
    let vec_actual: Vec<usize> = pritchard::n_primes_pritchard(num);
    assert_eq!(&vec_actual, vec_expected)
}

fn assert_segmented(num: usize, vec_expected: &Vec<usize>) {
    let vec_actual: Vec<usize> = segmented::n_primes_segmented(num).primes_result;
    assert_eq!(&vec_actual, vec_expected)
}

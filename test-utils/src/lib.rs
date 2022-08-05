use primal_sieve::Sieve;

pub fn assert_n<F>(num: usize, sieve: &Sieve, gen: F)
where
    F: FnOnce(usize) -> Vec<usize>,
{
    let expected: Vec<usize> = sieve.primes_from(0).take(num).collect();

    let result = gen(num);
    assert_eq!(result.as_slice(), expected.as_slice());
}

pub fn assert_primes<F>(num: usize, sieve: &Sieve, gen: F)
where
    F: FnOnce(usize) -> Vec<usize>,
{
    let expected: Vec<usize> = sieve.primes_from(0).take_while(|&n| n <= num).collect();
    let result = gen(num);
    assert_eq!(result.as_slice(), expected.as_slice());
}

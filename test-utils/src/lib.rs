use std::fs::File;
use std::io::BufWriter;
use std::time::SystemTime;

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

pub fn dump(sieve: &Sieve, vec: Vec<usize>) {
    let time = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis();

    let dump_expected = {
        let expected = format!("./prime_expected{}.json", time);
        BufWriter::new(File::create(expected).unwrap())
    };
    let dump_actual = {
        let actual = format!("./prime_actual{}.json", time);
        BufWriter::new(File::create(actual).unwrap())
    };
    serde_json::to_writer_pretty(dump_expected, &sieve.primes_from(0).collect::<Vec<usize>>()).unwrap();
    serde_json::to_writer_pretty(dump_actual, &vec).unwrap();
}

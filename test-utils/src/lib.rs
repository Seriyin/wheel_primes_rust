use std::fs::File;
use std::io::BufWriter;
use std::time::SystemTime;

use primal_sieve::Sieve;

pub const FIXED_SEED: [u8; 32] = [
    0x57, 0x4c, 0xbd, 0x96, 0x2d, 0x35, 0x60, 0x3e, 
    0x2c, 0x20, 0x91, 0x72, 0x05, 0xa3, 0xaf, 0xe1, 
    0x68, 0x5f, 0xa6, 0x4f, 0xdb, 0xd9, 0xac, 0xd0, 
    0x59, 0xd1, 0xc8, 0xc4, 0xd7, 0xd0, 0xee, 0x8e
];


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

use std::cmp::min;
use wasm_bindgen::prelude::*;

use crate::utils::{approximate_primes, isqrt};

#[wasm_bindgen]
pub fn sieve_pritchard_w(primes: usize) -> JsValue {
    JsValue::from_serde(&sieve_pritchard(primes)).unwrap()
}

pub fn sieve_pritchard(primes: usize) -> Vec<usize> {
    let mut start_primes: Vec<usize> = vec![2, 3];
    match primes {
        0 | 1 => {
            start_primes.clear();
            start_primes
        }
        2 => {
            start_primes.remove(1);
            start_primes
        }
        3 | 4 => start_primes,
        _ => {
            let mut wheel = Vec::<usize>::with_capacity(isqrt(primes));
            wheel.push(1);
            wheel.push(5);
            let mut length: usize = min(primes, 6);
            let mut p = 5;
            let mut primes_accum = start_primes;
            while p * p <= primes {
                if length < primes {
                    length = extend(&mut wheel, length, min(primes, p * length));
                }
                delete(&mut wheel, p);
                primes_accum.push(p);
                p = wheel[1];
            }
            if length < primes {
                extend(&mut wheel, length, primes);
            }
            union(&mut primes_accum, &wheel);
            primes_accum
        }
    }
}

#[wasm_bindgen]
pub fn n_primes_pritchard_w(primes: usize) -> JsValue {
    JsValue::from_serde(&n_primes_pritchard(primes)).unwrap()
}

pub fn n_primes_pritchard(n: usize) -> Vec<usize> {
    let approx = approximate_primes(n);

    let mut primes_accum = sieve_pritchard(approx);
    primes_accum.drain(n..);
    primes_accum
}

fn extend(wheel: &mut Vec<usize>, length: usize, n: usize) -> usize {
    let mut i = 1;
    let mut w;
    let mut x = length + 1;
    while x <= n {
        wheel.push(x);
        w = wheel[i];
        x = w + length;
        i += 1;
    }
    n
}

fn delete(wheel: &mut Vec<usize>, p: usize) {
    wheel.retain(|w| w % p != 0);
}

fn union(prk: &mut Vec<usize>, wheel: &[usize]) {
    for i in &wheel[1..] {
        prk.push(*i);
    }
}

#[cfg(test)]
mod tests {
    use primal_sieve::Sieve;


    use test_utils::assert_primes;

    #[test]
    fn wheel_fact_by_limit() {
        use super::sieve_pritchard;

        let primes: Sieve = Sieve::new(2000);


        assert_primes(5, &primes, sieve_pritchard);
        assert_primes(2000, &primes, sieve_pritchard);
    }


    use test_utils::assert_n;

    #[test]
    fn wheel_fact_by_n() {
        use super::n_primes_pritchard;

        let primes: Sieve = Sieve::new(2000);

        assert_n(3, &primes, n_primes_pritchard);
        assert_n(303, &primes, n_primes_pritchard);
    }

}

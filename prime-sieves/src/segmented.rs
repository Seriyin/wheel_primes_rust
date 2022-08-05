use wasm_bindgen::prelude::*;
use std::cmp::min;
use std::cmp::max;

use crate::utils::approximate_primes;
use crate::utils::isqrt;


const L1D_CACHE_SIZE: usize = 32768;

#[derive(Debug)]
pub struct SieveSegmented {
    primes: usize,
    sqrt: usize,
    count: usize,
    segment_size: usize,
    is_prime: Vec<bool>,
    primes_vec: Vec<usize>,
    multiples: Vec<usize>,
    pub primes_result: Vec<usize>
}

impl SieveSegmented {
    fn empty() -> SieveSegmented {
        SieveSegmented {
            primes: 0,
            sqrt: 0,
            count: 0,
            segment_size: 0,
            is_prime: vec![],
            primes_vec: vec![],
            multiples: vec![],
            primes_result: vec![],
        }
    }

    fn single() -> SieveSegmented {
        SieveSegmented {
            primes: 1,
            sqrt: 1,
            count: 1,
            segment_size: 0,
            is_prime: vec![],
            primes_vec: vec![],
            multiples: vec![],
            primes_result: vec![2],
        } 
    }

    fn new_n(n: usize) -> SieveSegmented {
        let primes = approximate_primes(n);
        let sqrt: usize = isqrt(primes);
        return SieveSegmented { 
            primes,
            sqrt,
            count: 0,
            segment_size: max(sqrt, L1D_CACHE_SIZE),
            is_prime: vec!(true; sqrt + 1), 
            primes_vec: Vec::with_capacity(sqrt), 
            multiples: Vec::with_capacity(sqrt),
            primes_result: Vec::with_capacity(n * 5 / 4)
        }
    }

    fn new(primes: usize) -> SieveSegmented {
        let sqrt: usize = isqrt(primes);
        return SieveSegmented {
            primes,
            sqrt,
            count: 0,
            segment_size: max(sqrt, L1D_CACHE_SIZE),
            is_prime: vec!(true; sqrt + 1),
            primes_vec: Vec::with_capacity(sqrt),
            multiples: Vec::with_capacity(sqrt),
            primes_result: Vec::with_capacity(sqrt)
        }
       
    }

    fn calculate_primes_for_segment(
        &mut self, low: usize, mut i: usize, mut n: usize, mut s: usize, 
        sieve_segment: &mut Vec<bool>
    ) -> (usize, usize, usize) {
        sieve_segment.fill(true);
    
        // current segment = [low, high]
        let high = min(low + self.segment_size - 1, self.primes);

        // generate sieving primes using simple sieve of Eratosthenes
        while i * i <= high {
            if self.is_prime[i] {
                let mut j = i * i;
                while j <= self.sqrt {
                    self.is_prime[j] = false;
                    j += i;
                }
            }
            i += 2
        }

        // initialize sieving primes for segmented sieve
        while s * s <= high
        {
            if self.is_prime[s] {
                self.primes_vec.push(s);
                self.multiples.push(s * s - low);
            }
            s += 2
        }

        // sieve the current segment
        for i in 0..self.primes_vec.len() {
            let mut j: usize = self.multiples[i];
            let k = self.primes_vec[i] * 2;
            while j < self.segment_size {
                sieve_segment[j] = false;
                j += k;
            }
            self.multiples[i] = j - self.segment_size;
        }

        while n <= high {
            if sieve_segment[n - low] {
                self.primes_result.push(n);
                self.count += 1;
            } // n is a prime
            n += 2
        }

        return (i, n, s)

    
    }

    fn sieve_segmented_loop(&mut self) {
        let mut sieve_segment = vec!(true; self.segment_size);
        self.primes_result.push(2);
        self.count = 1;

        let mut i = 3;
        let mut n = 3;
        let mut s = 3;
    
        let mut low: usize = 0;
    
    
        while low <= self.primes {
            (i, n, s) = self.calculate_primes_for_segment(low, i, n, s, &mut sieve_segment);
    
            low += self.segment_size;
        }
            
    }
}

#[wasm_bindgen]
pub fn sieve_segmented_w(primes: usize) -> JsValue {
    let sieve_segmented = sieve_segmented(primes);

    JsValue::from_serde(&sieve_segmented.primes_result).unwrap()
}

pub fn sieve_segmented(primes: usize) -> SieveSegmented {
    match primes {
        0 | 1 => SieveSegmented::empty(),
        2 => SieveSegmented::single(),
        _ => {
            let mut sieve_segmented = SieveSegmented::new(primes);
            sieve_segmented.sieve_segmented_loop();
            sieve_segmented    
        }
    }
}


#[wasm_bindgen]
pub fn n_primes_segmented_w(primes: usize) -> JsValue {
    let sieve_segmented = n_primes_segmented(primes);

    JsValue::from_serde(&sieve_segmented.primes_result).unwrap()
}

pub fn n_primes_segmented(primes: usize) -> SieveSegmented {
    match primes {
        0 => SieveSegmented::empty(),
        1 => SieveSegmented::single(),
        _ => {
            let mut sieve_segmented = SieveSegmented::new_n(primes);
            sieve_segmented.sieve_segmented_loop();
    
            sieve_segmented.primes_result.drain(primes..);
    
            sieve_segmented
        }

    }
}

#[cfg(test)]
mod tests {
    use primal_sieve::Sieve;


    use test_utils::assert_primes;

    #[test]
    fn segmented_fact_by_limit() {
        use super::sieve_segmented;

        let primes: Sieve = Sieve::new(2000);


        assert_primes(5, &primes, |n| sieve_segmented(n).primes_result);
        assert_primes(2000, &primes, |n| sieve_segmented(n).primes_result);
    }

    use test_utils::assert_n;

    #[test]
    fn segmented_fact_by_n() {
        use super::n_primes_segmented;

        let primes: Sieve = Sieve::new(2000);

        assert_n(3, &primes, |n| n_primes_segmented(n).primes_result);
        assert_n(303, &primes, |n| n_primes_segmented(n).primes_result);
    }
}

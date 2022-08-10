use bitvec::prelude::{BitSlice};
use bitvec::vec::BitVec;
use wasm_bindgen::prelude::*;
use std::cmp::min;
use std::cmp::max;

use crate::utils::approximate_primes;
use crate::utils::isqrt;


//Approximate size of L1 Cache in bits x2 because
//even numbers are not stored.
const L1D_CACHE_SIZE: usize = 32768 * 8 * 2;

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
        SieveSegmented { 
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
        SieveSegmented {
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
        &mut self, low: usize, mut i: usize, mut s: usize, 
        sieve_segment: &mut BitSlice
    ) -> (usize, usize) {
    
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
                if j % 2 == 1 {
                    //replace unchecked is perfectly fine due to bounds [0, self.segment_size] being perfectly defined a priori.
                    //j is always smaller than self.segment_size as per while condition.
                    unsafe {
                        sieve_segment.replace_unchecked(j / 2, false)  
                    };
                }
                j += k;
            }
            self.multiples[i] = j - self.segment_size;
        }

        let diff = (high - low - 1) / 2;
        sieve_segment[..=diff].iter_ones().for_each(
            |i| {
                self.primes_result.push(low + i * 2 + 1);
                self.count += 1;
            }
        );

        sieve_segment.fill(true);

        (i, s)

    
    }

    fn sieve_segmented_loop(&mut self) {
        let vec = vec![usize::MAX; self.segment_size / (usize::BITS as usize)];
        let mut sieve_segment = BitVec::from_vec(vec).into_boxed_bitslice();
        assert_eq!(sieve_segment.len(), self.segment_size);

        self.primes_result.push(2);
        self.count = 1;

        let mut i = 3;
        let mut s = 3;
    
        let mut low: usize = 0;

        //replace unchecked is perfectly fine due to bounds [0, self.segment_size] being perfectly defined a priori.
        unsafe {
            sieve_segment.replace_unchecked(0, false);
        }
    
    
        while low < self.primes {
            (i, s) = self.calculate_primes_for_segment(low, i, s, sieve_segment.as_mut_bitslice());
    
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
            //Circumvents non-primes or primes within segment bigger than primes.
            /* 
            let pop_cnt = {
                let primes_result = &sieve_segmented.primes_result;
                let lngth = primes_result.len();
                let mut i = 0;
                while lngth - 1 - i > 0 && primes_result[lngth - 1 - i] > primes {
                    i += 1
                };
                i
            };
            {
                for _i in 0..pop_cnt {
                    let primes_result = &mut sieve_segmented.primes_result;
                    primes_result.pop();
                }    
            }
            */
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
//    use test_utils::dump;

    #[test]
    fn segmented_fact_by_limit() {
        use super::sieve_segmented;


        let primes_test = [5, 2000, 16777216];
        let primes: Sieve = Sieve::new(*primes_test.iter().max().unwrap());
        for i in primes_test {
            assert_primes(i, &primes, |n| sieve_segmented(n).primes_result);
        }
//        dump(&Sieve::new(15179958), sieve_segmented(15179958).primes_result);
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

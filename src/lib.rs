use std::cmp::min;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sieve_pritchard(primes: usize) -> JsValue {
    let start_primes: Vec<usize> = vec![1, 2, 3];
    if primes < 5 {
        let index = start_primes.iter().enumerate().rev().find(
            |&(_, &num)| num < primes
        ).get_or_insert((0, &0)).0;
        JsValue::from_serde(&start_primes[..index]).unwrap()
    } else {
        let mut wheel = Vec::<usize>::with_capacity(isqrt(primes));
        wheel.push(1); wheel.push(5);
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
        JsValue::from_serde(&primes_accum).unwrap()
    }
}

#[wasm_bindgen]
pub fn n_primes_pritchard(primes: usize) -> JsValue {
    let start_primes: Vec<usize> = vec![1, 2, 3];
    if primes <= 3 {
        JsValue::from_serde(&start_primes[..primes]).unwrap()
    } else {
        let mut wheel = Vec::<usize>::with_capacity(isqrt(primes));
        let mut primes_accum = start_primes;
        wheel.push(1); wheel.push(5);
        let mut length: usize = 6;
        let mut p = 5;
        while primes_accum.len() + wheel.len() < primes + 1 {
            length = extend(&mut wheel, length, p * length);
            
            delete(&mut wheel, p);
            primes_accum.push(p);
            p = wheel[1];
        }
        union(&mut primes_accum, &wheel);
        


        JsValue::from_serde(&primes_accum[..primes]).unwrap()
    }
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

fn isqrt(n: usize) -> usize {
    // Xₙ₊₁
    let mut x = n;

    // cₙ
    let mut c = 0;


    // dₙ which starts at the highest power of four <= n
    let mut d = 1 << usize::BITS - 2; // The second-to-top bit is set.
                            // Same as ((unsigned) INT32_MAX + 1) / 2.
    while d > n {
        d >>= 2;
    }

    // for dₙ … d₀
    while d != 0 {
        if x >= c + d {      // if Xₘ₊₁ ≥ Yₘ then aₘ = 2ᵐ
            x -= c + d;        // Xₘ = Xₘ₊₁ - Yₘ
            c = (c >> 1) + d;  // cₘ₋₁ = cₘ/2 + dₘ (aₘ is 2ᵐ)
        }
        else {
            c >>= 1;           // cₘ₋₁ = cₘ/2      (aₘ is 0)
        }
        d >>= 2;               // dₘ₋₁ = dₘ/4
    }
    return c.try_into().unwrap();                  // c₋₁

}

fn union(prk: &mut Vec<usize>, wheel: &Vec<usize>) {
    for i in &wheel[1..] {
        prk.push(*i);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

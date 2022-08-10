pub fn isqrt(n: usize) -> usize {
    // Xₙ₊₁
    let mut x = n;

    // cₙ
    let mut c = 0;

    // dₙ which starts at the highest power of four <= n
    let mut d = 1 << (usize::BITS - 2); // The second-to-top bit is set.
                            // Same as ((unsigned) usize::MAX + 1) / 2.
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
    c                  // c₋₁

}

pub fn approximate_primes(n: usize) -> usize {
    let approx_log = (usize::BITS - usize::leading_zeros(n)) as usize;
    approx_log * n
}

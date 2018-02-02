use std::num::Wrapping;

// Based on num_traits::checked_pow
pub(crate) fn wrapping_pow(base: u64, mut exp: usize) -> u64 {
    if exp == 0 { return 1; }
    
    let mut base = Wrapping(base);
    
    while exp & 1 == 0 {
        base *= base;
        exp >>= 1;
    }

    if exp == 1 {
        return base.0;
    }

    let mut acc = base;
    while exp > 1 {
        exp >>= 1;
        base *= base;
        if exp & 1 == 1 {
            acc *= base;
        }
    }
    acc.0
}
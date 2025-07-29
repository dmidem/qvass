//! Provides classical number theory helper functions.
//!
//! These functions are essential for the classical pre- and post-processing steps
//! in quantum algorithms like Shor's algorithm for integer factorization.

/// Computes `(a ^ b) mod m` using binary exponentiation (also known as exponentiation by squaring).
/// This is efficient for large exponents
pub fn powmod(mut a: u64, b: u64, m: u64) -> u64 {
    assert!(m > 0, "Modulus must be positive");
    a %= m;
    let mut result: u64 = if a != 0 { 1 } else { 0 };
    let mut base = a;
    let mut exponent = b;

    while exponent > 0 {
        if exponent & 1 != 0 {
            result = result.wrapping_mul(base) % m;
        }
        base = base.wrapping_mul(base) % m;
        exponent >>= 1;
    }

    result
}

/// Computes the greatest common divisor (GCD) of two integers using the Euclidean algorithm
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        [a, b] = [b, a % b];
    }
    a.abs()
}

/// Computes the modular multiplicative inverse of `a` modulo `m`.
///
/// This function finds an integer `x` such that `(a * x) mod m = 1`.
///
/// # Panics
/// Panics if `a` and `m` are not coprime (i.e., their GCD is not 1),
/// as no modular inverse exists in that case. Also panics if `a` or `m` are not positive.
pub fn invmod(a: i64, m: i64) -> i64 {
    assert!(a > 0 && m > 0, "Both a and m must be positive");
    let (gcd_val, x, _) = extended_gcd(a, m);
    assert!(gcd_val == 1, "a and m must be coprime");

    // Ensure the inverse is positive
    (x % m + m) % m
}

/// Performs the Extended Euclidean algorithm.
///
/// Returns a tuple `(gcd, x, y)` that satisfies Bézout's identity: `a * x + b * y = gcd(a, b)`.
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        (gcd, y1, x1 - (a / b) * y1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powmod() {
        assert_eq!(powmod(2, 10, 1000), 24); // 2^10 = 1024 mod 1000 = 24
        assert_eq!(powmod(3, 0, 5), 1); // 3^0 mod 5 = 1
        assert_eq!(powmod(0, 5, 7), 0); // 0^5 mod 7 = 0
        assert_eq!(powmod(10, 9, 6), 4); // 10^9 mod 6 = 4
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(-54, 24), 6);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(0, 0), 0); // Edge case: gcd(0, 0) is undefined, but returns 0
    }

    #[test]
    fn test_invmod() {
        assert_eq!(invmod(3, 11), 4); // 3 * 4 mod 11 = 12 mod 11 = 1
        assert_eq!(invmod(10, 17), 12); // 10 * 12 mod 17 = 120 mod 17 = 1
        assert_eq!(invmod(7, 26), 15); // 7 * 15 mod 26 = 105 mod 26 = 1

        // TODO:
        // Test inverse when a and m are not coprime (should panic)
        // Uncommenting the following line will cause a panic
        // invmod(2, 4);
    }
}

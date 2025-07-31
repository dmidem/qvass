//! Classical number theory helper functions.

/// Computes `(a ^ b) mod m` using binary exponentiation.
pub fn powmod(mut a: u64, b: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return None;
    }

    a %= m;
    let mut result = 1u64;
    let mut base = a;
    let mut exponent = b;

    while exponent > 0 {
        if exponent & 1 != 0 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exponent >>= 1;
    }

    Some(result)
}

/// Computes the greatest common divisor using the Euclidean algorithm.
pub fn gcd(a: i64, b: i64) -> i64 {
    // Convert to u64 to avoid i64::MIN.abs() overflow
    let mut a = a.unsigned_abs();
    let mut b = b.unsigned_abs();

    while b != 0 {
        [a, b] = [b, a % b];
    }
    a as i64
}

/// Computes the modular multiplicative inverse of `a` modulo `m`.
/// Returns `x` such that `(a * x) mod m = 1`, or None if no inverse exists.
pub fn invmod(a: u64, m: u64) -> Option<u64> {
    if a == 0 || m == 0 {
        return None;
    }

    let (g, x, _) = extended_gcd(a as i64, m as i64);

    if g != 1 {
        return None; // No inverse exists
    }

    // Normalize to positive result
    Some(((x % m as i64 + m as i64) % m as i64) as u64)
}

/// Extended Euclidean algorithm.
/// Returns `(gcd, x, y)` satisfying Bézout's identity: `a * x + b * y = gcd(a, b)`.
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        let abs_a = if a == i64::MIN { i64::MAX } else { a.abs() };
        (abs_a, if a >= 0 { 1 } else { -1 }, 0)
    } else {
        let (g, x1, y1) = extended_gcd(b, a % b);
        // Use i128 for intermediate calculation to prevent overflow
        let x = y1;
        let y = x1 - (a as i128 / b as i128 * y1 as i128) as i64;
        (g, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powmod() {
        assert_eq!(powmod(2, 10, 1000), Some(24)); // 2^10 = 1024 mod 1000 = 24
        assert_eq!(powmod(3, 0, 5), Some(1)); // Any number^0 = 1
        assert_eq!(powmod(0, 5, 7), Some(0)); // 0^n = 0 for n > 0
        assert_eq!(powmod(0, 0, 7), Some(1)); // 0^0 = 1 by convention
        assert_eq!(powmod(10, 9, 6), Some(4)); // 10^9 mod 6 = 4
        assert_eq!(powmod(5, 3, 0), None); // Division by zero

        // Test overflow protection
        assert_eq!(powmod(u64::MAX, 2, 1000), Some(225)); // (2^64-1)^2 mod 1000
        assert_eq!(powmod(999, u64::MAX, 1000), Some(999)); // Large exponent
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(-54, 24), 6); // Handles negative numbers
        assert_eq!(gcd(24, -54), 6);
        assert_eq!(gcd(-24, -54), 6); // Both negative
        assert_eq!(gcd(0, 5), 5); // gcd(0, n) = |n|
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(17, 13), 1); // Coprime numbers
        assert_eq!(gcd(0, 0), 0); // Edge case

        // Test overflow edge cases
        assert_eq!(gcd(i64::MIN, 6), 2); // i64::MIN is even, so gcd with 6 includes factor of 2
        assert_eq!(gcd(6, i64::MIN), 2);
        assert_eq!(gcd(i64::MAX, 1), 1);
    }

    #[test]
    fn test_invmod() {
        assert_eq!(invmod(3, 11), Some(4)); // 3 * 4 ≡ 1 (mod 11)
        assert_eq!(invmod(10, 17), Some(12)); // 10 * 12 ≡ 1 (mod 17)
        assert_eq!(invmod(7, 26), Some(15)); // 7 * 15 ≡ 1 (mod 26)

        // Test cases with no inverse
        assert_eq!(invmod(2, 4), None); // gcd(2, 4) = 2 ≠ 1
        assert_eq!(invmod(6, 9), None); // gcd(6, 9) = 3 ≠ 1
        assert_eq!(invmod(0, 5), None); // 0 has no inverse
        assert_eq!(invmod(5, 0), None); // Modulus cannot be 0

        // Edge cases
        assert_eq!(invmod(1, 5), Some(1)); // 1 is its own inverse

        // Verify correctness
        if let Some(inv) = invmod(123, 1000) {
            assert_eq!((123u64 * inv) % 1000, 1);
        }
    }

    #[test]
    fn test_extended_gcd() {
        let (g, x, y) = extended_gcd(30, 18);
        assert_eq!(g, 6);
        assert_eq!(30 * x + 18 * y, g); // Verify Bézout's identity

        let (g, x, y) = extended_gcd(17, 13);
        assert_eq!(g, 1);
        assert_eq!(17 * x + 13 * y, g);

        // Test with negative numbers
        let (g, x, y) = extended_gcd(-30, 18);
        assert_eq!(g, 6);
        assert_eq!(-30 * x + 18 * y, g);

        let (g, x, y) = extended_gcd(240, 46);
        assert_eq!(g, 2);
        assert_eq!(240 * x + 46 * y, g);
    }
}

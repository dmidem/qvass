//! Provides common utilities and assertion functions for the crate's test suite.

pub(crate) mod vectors;

use std::{format, string::String, vec::Vec};

use num_complex::Complex64;

// Formats a state vector slice into a human-readable string for debugging.
fn fmt_state(slice: &[Complex64]) -> String {
    slice
        .iter()
        .map(|c| format!("{:.6}{:+.6}i", c.re, c.im))
        .collect::<Vec<_>>()
        .join(", ")
}

// Asserts that two state vectors are approximately equal within a tolerance.
//
// Panics with a formatted message including the provided `context` if they are not.
pub(crate) fn assert_state_eq(actual: &[Complex64], expected: &[Complex64]) {
    let tolerance = 1e-6;

    assert_eq!(
        expected.len(),
        actual.len(),
        "Vectors have different lengths: expected {}, got {}",
        expected.len(),
        actual.len()
    );

    assert!(
        expected
            .iter()
            .zip(actual.iter())
            .all(|(exp, act)| (*exp - *act).l1_norm() < tolerance),
        "Vectors differs:\n  actual: [{}]\n  expect: [{}]",
        fmt_state(actual),
        fmt_state(expected),
    );
}

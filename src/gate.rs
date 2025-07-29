//! Defines the `Gate` enum, which represents all possible quantum operations.
//!
//! This module provides a composable API for creating and manipulating quantum gates.
//! Gates can be basic (like Hadamard or NOT), parameterized (Phase), or composite
//! (Controlled gates or entire Circuits treated as a single gate).

use core::f64::consts::{FRAC_1_SQRT_2, PI};

use alloc::boxed::Box;

use num_complex::Complex64;

use super::circuit::Circuit;

/// Represents different types of quantum gates
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Gate {
    /// Hadamard gate - creates superposition
    Hadamard,
    /// Pauli-X gate (NOT gate) - bit flip
    Not,
    /// Phase gate with complex phase factor
    Phase(Complex64),
    /// SWAP gate - swaps two qubits
    Swap,
    /// Controlled version of any gate
    Controlled(Box<Gate>),
    /// Nested circuit as a gate
    Circuit(Circuit),
}

impl Gate {
    /// Creates a Hadamard gate
    #[inline]
    pub fn hadamard() -> Self {
        Self::Hadamard
    }

    /// Creates a NOT (Pauli-X) gate
    #[inline]
    pub fn not() -> Self {
        Self::Not
    }
    /// Creates a phase gate with the given angle in radians
    #[inline]
    pub fn phase_radians(angle: f64) -> Self {
        Self::Phase(Complex64::new(0.0, angle).exp())
    }

    /// Creates a phase gate with the given fraction of 2π
    #[inline]
    pub fn phase_fraction(fraction: f64) -> Self {
        Self::phase_radians(2.0 * PI * fraction)
    }

    /// Creates a SWAP gate
    #[inline]
    pub fn swap() -> Self {
        Self::Swap
    }

    /// Creates a controlled version of the given gate
    #[inline]
    pub fn control(self) -> Self {
        Self::Controlled(Box::new(self))
    }
    /// Creates a multi-controlled version of the given gate
    #[inline]
    pub fn multi_control(self, n: u8) -> Self {
        (0..n).fold(self, |gate, _| gate.control())
    }

    /// Creates a gate from a circuit
    #[inline]
    pub fn circuit(circuit: Circuit) -> Self {
        Self::Circuit(circuit)
    }

    /// Creates a CNOT gate (controlled NOT)
    #[inline]
    pub fn cnot() -> Self {
        Self::Not.control()
    }

    /// Creates a Toffoli gate (controlled CNOT)
    #[inline]
    pub fn toffoli() -> Self {
        Self::cnot().control()
    }

    /// Creates a Fredkin gate (controlled SWAP)
    #[inline]
    pub fn fredkin() -> Self {
        Self::swap().control()
    }

    /// Recursively counts the total number of control qubits for this gate
    #[inline]
    pub fn count_controlled(&self) -> u8 {
        if let Self::Controlled(inner) = self {
            inner.count_controlled() + 1
        } else {
            0
        }
    }

    /// Returns the inverse (adjoint) of this gate
    #[inline]
    pub fn inverse(&self) -> Self {
        match self {
            // Self-adjoint gates.
            Self::Hadamard => Self::Hadamard,
            Self::Not => Self::Not,
            Self::Swap => Self::Swap,

            // The inverse of a phase rotation is the conjugate rotation
            Self::Phase(c) => Self::Phase(c.conj()),

            // Recursively find the inverse of composite gates
            Self::Controlled(inner_gate) => Self::Controlled(Box::new(inner_gate.inverse())),
            Self::Circuit(circuit) => Self::Circuit(circuit.inverse()),
        }
    }

    /// Applies the gate's transformation to a local sub-state
    pub fn apply(&self, state: &mut [Complex64]) {
        // TODO: Call validate_state_normalization?

        match self {
            Self::Hadamard => {
                debug_assert!(
                    state.len() >= 2,
                    "Hadamard gate requires at least 2 amplitudes"
                );
                let (a, b) = (state[0], state[1]);
                state[0] = FRAC_1_SQRT_2 * (a + b);
                state[1] = FRAC_1_SQRT_2 * (a - b);
            }
            Self::Not => {
                debug_assert!(state.len() >= 2, "NOT gate requires at least 2 amplitudes");
                state.swap(0, 1);
            }
            Self::Swap => {
                debug_assert!(state.len() >= 3, "SWAP gate requires at least 3 amplitudes");
                state.swap(1, 2);
            }
            Self::Phase(phase) => {
                debug_assert!(
                    state.len() >= 2,
                    "Phase gate requires at least 2 amplitudes"
                );
                state[1] *= phase;
            }
            Self::Controlled(gate) => {
                let substate_middle = state.len() >> 1;
                gate.apply(&mut state[substate_middle..])
            }
            Self::Circuit(circuit) => {
                circuit.apply(state);
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::println;

    use crate::tests::{
        assert_state_eq,
        vectors::{self, TestVector},
    };

    use super::*;

    // Validates that the quantum state has the correct normalization
    fn validate_state_normalization(state: &[Complex64], tolerance: f64) -> bool {
        let norm_squared: f64 = state.iter().map(|amp| amp.norm_sqr()).sum();
        (norm_squared - 1.0).abs() < tolerance
    }

    // Runs tests for gates with custom gate construction
    pub(crate) fn run_parameterized_gate_tests<'a, Args: 'a>(
        title: &str,
        build_gate: impl Fn(&'a TestVector<Args>) -> Gate,
        test_vectors: impl IntoIterator<Item = &'a TestVector<Args>>,
    ) {
        const STATE_NORM_TOLERANCE: f64 = 1E-6;

        for (i, test_vector) in test_vectors.into_iter().enumerate() {
            println!("{title} circuit test case #{i}");

            // FIXME: Avoid test_vector.qubits.iter().copied()?
            let circuit =
                Circuit::from_gate(build_gate(test_vector), test_vector.qubits.iter().copied())
                    .unwrap();

            let mut state = test_vector.initial_state.clone();

            validate_state_normalization(&test_vector.expected_state, STATE_NORM_TOLERANCE);
            validate_state_normalization(&state, STATE_NORM_TOLERANCE);

            circuit.apply(&mut state);

            assert_state_eq(&state, &test_vector.expected_state);

            // Roundtrip test: Apply the inverse and verify it returns to the initial state
            let inverse_circuit = circuit.inverse();
            inverse_circuit.apply(&mut state);
            assert_state_eq(&state, &test_vector.initial_state);
        }
    }

    // Runs tests for gates that don't require parameters
    pub(crate) fn run_gate_tests<'a, Args: 'a>(
        title: &str,
        gate: Gate,
        test_vectors: impl IntoIterator<Item = &'a TestVector<Args>>,
    ) {
        run_parameterized_gate_tests(title, |_| gate.clone(), test_vectors)
    }

    #[test]
    fn hadamard_gate() {
        run_gate_tests("HADAMARD", Gate::hadamard(), &*vectors::HADAMARD_TESTS);
    }

    #[test]
    fn not_gate() {
        run_gate_tests("NOT", Gate::not(), &*vectors::NOT_TESTS);
    }

    #[test]
    fn phase_gate() {
        run_parameterized_gate_tests(
            "PHASE",
            |test_vector| Gate::phase_fraction(test_vector.args.fraction),
            &*vectors::PHASE_TESTS,
        );
    }

    #[test]
    fn cnot_gate() {
        run_gate_tests("CNOT", Gate::cnot(), &*vectors::CNOT_TESTS);
    }

    #[test]
    fn swap_gate() {
        run_gate_tests("SWAP", Gate::swap(), &*vectors::SWAP_TESTS);
    }

    #[test]
    fn toffoli_gate() {
        run_gate_tests("TOFFOLI", Gate::toffoli(), &*vectors::TOFFOLI_TESTS);
    }

    #[test]
    fn fredkin_gate() {
        run_gate_tests("FREDKIN", Gate::fredkin(), &*vectors::FREDKIN_TESTS);
    }
}

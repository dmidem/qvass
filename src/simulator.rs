//! Defines the main `QuantumSimulator`, which manages circuit construction,
//! state vector evolution, and measurement.
//!
//! This module provides the primary interface for users to build and execute
//! quantum simulations.

use alloc::{fmt, vec, vec::Vec};

use rand::Rng;

use num_complex::Complex64;

use super::{
    circuit::{Circuit, QubitError, QubitIndices},
    gate::Gate,
};

/// Represents the state vector of a quantum system.
pub struct QuantumState(Vec<Complex64>);

impl AsRef<[Complex64]> for QuantumState {
    fn as_ref(&self) -> &[Complex64] {
        &self.0
    }
}

impl fmt::Display for QuantumState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_assert!(
            self.0.len().is_power_of_two(),
            "QuantumState length must be a non-zero power of two, but was {}",
            self.0.len()
        );

        let bits = self.0.len().trailing_zeros() as usize;
        for i in 0..self.0.len() {
            let amplitude = self.0[i];
            write!(
                f,
                "|{:0bits$b}⟩: ({:>9.6}, {:>9.6})",
                i, amplitude.re, amplitude.im
            )?;
        }
        Ok(())
    }
}

/// Manages the state of a quantum system and the application of a circuit
pub struct QuantumSimulator {
    circuit: Circuit,
    state: QuantumState, // Size = 2^n_qubits
}

impl QuantumSimulator {
    /// Creates a new quantum simulator for a system of `n_qubits`.
    ///
    /// The simulator is initialized with an empty circuit and a state vector
    /// corresponding to the |0...0⟩ basis state, although the state is zeroed.
    /// Use `init_state` to prepare a specific initial state.
    pub fn new(n_qubits: u8) -> Self {
        assert!(n_qubits > 0, "Circuit should have at least one qubit");

        let state_size = 1 << n_qubits;
        let state_data = vec![Complex64::ZERO; state_size];

        Self {
            circuit: Circuit::new(n_qubits),
            state: QuantumState(state_data),
        }
    }

    /// Adds a gate to the simulator's internal circuit
    pub fn add_gate<I: QubitIndices>(
        &mut self,
        gate: Gate,
        qubit_indices: I,
    ) -> Result<(), QubitError> {
        self.circuit.add_gate(gate, qubit_indices)
    }

    /// Resets and initializes the state vector to a specific computational basis state.
    pub fn init_state(&mut self, one_index: usize) {
        self.state.0.fill(Complex64::ZERO);
        self.state.0[one_index] = Complex64::ONE;
    }

    /// Applies the accumulated circuit to the current state vector.
    pub fn run(&mut self) {
        self.circuit.apply(&mut self.state.0);
    }

    /// Performs a measurement on the final state vector.
    ///
    /// The outcome is determined probabilistically based on the amplitudes of the state vector.
    /// After measurement, the system's state collapses to the measured basis state.
    pub fn measure<R: Rng>(&mut self, rng: &mut R) -> usize {
        let probe: f64 = rng.gen();

        // Find the outcome by iterating through cumulative probabilities
        let outcome_index = self
            .state
            .0
            .iter()
            .scan((0f64, 0usize), |(sum, index), &a| {
                if *sum > probe {
                    None
                } else {
                    let result = *index;

                    *sum += a.norm_sqr();
                    *index += 1;

                    Some(result)
                }
            })
            .last()
            .unwrap_or(self.state.0.len() - 1);

        // Collapse the state to the measured outcome
        self.init_state(outcome_index);

        outcome_index
    }

    /// Returns an immutable reference to the simulator's internal state vector
    pub fn state(&self) -> &QuantumState {
        &self.state
    }
}

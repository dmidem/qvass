//! Defines the `Circuit` structure, which represents a sequence of quantum gates
//! applied to a set of qubits.
//!
//! This module provides the core logic for constructing and applying quantum circuits
//! to a state vector. It includes an optimized `MappedGate` implementation that
//! remaps qubit indices for efficient application, particularly for controlled gates.

use core::{error, fmt};

use alloc::{vec, vec::Vec};

use num_complex::Complex64;

use super::gate::Gate;

// The state size is 2^number of qubits, so limit the number to protect from memory overflow
const MAX_QUBITS: u8 = 32;

/// Represents errors that can occur during circuit construction
#[derive(Debug)]
pub enum QubitError {
    /// Indicates that a qubit index is outside the valid range for the circuit
    IndexOutOfBounds,
    /// Indicates that the same qubit index was used multiple times for a single gate
    DuplicatedIndex,
}

/// A trait for types that can be converted into an iterator over qubit indices
impl fmt::Display for QubitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QubitError::IndexOutOfBounds => write!(f, "Index is out of bounds"),
            QubitError::DuplicatedIndex => write!(f, "Duplicated index"),
        }
    }
}

impl error::Error for QubitError {}

pub trait QubitIndices: IntoIterator<Item = u8> {
    type Iter: ExactSizeIterator<Item = u8>;

    fn into_count_iter(self) -> (usize, Self::Iter);
}

impl<T> QubitIndices for T
where
    T: IntoIterator<Item = u8>,
    T::IntoIter: ExactSizeIterator,
{
    type Iter = T::IntoIter;

    fn into_count_iter(self) -> (usize, Self::Iter) {
        let iter = self.into_iter();
        let count = iter.len();

        (count, iter)
    }
}

/// Gate with a map between its qubit local state indices and full state indices
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
struct MappedGate {
    /// Gate kernel
    gate: Gate,

    /// Maps local indices to global state indices
    state_map: Vec<usize>,

    /// Bitmask for target qubits
    qubits_mask: usize,
}

impl MappedGate {
    // Remap qubit indices for efficient controlled-gate application.
    //
    // Qiskit's convention is `[controls..., targets...]`. To optimize, we rearrange
    // the logical bit order by moving control bits to the most significant positions
    // while also reversing their sequence.
    //
    // Example: `[c0, c1, t0, t1]` -> logical bits `[t0, t1, c1, c0]`
    //
    // This layout packs all control-active states into the upper half of the local
    // state vector. As a result, applying control gate becomes a single, fast slice
    // operation (`state[mid..]`) as seen in `Gate::apply`, avoiding a complex
    // scatter-gather process (i.e., updating elements that are interleaved with
    // unaffected ones).
    //
    // See Qiskit's convention: https://docs.quantum.ibm.com/guides/bit-ordering#gates
    fn calc_local_bit_pos(input_index: usize, qubit_count: usize, n_controlled: usize) -> usize {
        if input_index < n_controlled {
            // Map the j-th control to its MSB position, which reverses the original sequence.
            qubit_count - input_index - 1
        } else {
            // Map the target to its LSB position, preserving the original sequence.
            input_index - n_controlled
        }
    }

    fn new<I: QubitIndices>(
        gate: Gate,
        qubit_indices: I,
        max_qubits: Option<u8>,
    ) -> Result<Self, QubitError> {
        let (qubit_count, qubit_iter) = qubit_indices.into_count_iter();

        let mut state_map = vec![0; 1 << qubit_count];
        let mut qubits_mask = 0;

        let n_controlled = gate.count_controlled() as usize;

        for (input_index, qubit_index) in qubit_iter.enumerate() {
            let local_bit_pos = Self::calc_local_bit_pos(input_index, qubit_count, n_controlled);

            if let Some(max_qubits) = max_qubits {
                if qubit_index >= max_qubits {
                    return Err(QubitError::IndexOutOfBounds);
                }
            }

            let qubit_mask = 1 << qubit_index;

            if qubits_mask & qubit_mask != 0 {
                return Err(QubitError::DuplicatedIndex);
            }

            qubits_mask |= qubit_mask;

            for (i, inner_index) in state_map.iter_mut().enumerate() {
                if (i & (1 << local_bit_pos)) != 0 {
                    *inner_index |= qubit_mask;
                }
            }
        }

        Ok(Self {
            gate,
            state_map,
            qubits_mask,
        })
    }

    fn apply(&self, state: &mut [Complex64]) {
        let mut substate = vec![Complex64::ZERO; self.state_map.len()];
        let mut outer_index = 0;

        // For each substate block, extract substate, apply the gate, and write the substate back
        for _ in 0..state.len() / substate.len() {
            for (i, inner_index) in self.state_map.iter().enumerate() {
                substate[i] = state[outer_index | inner_index];
            }

            self.gate.apply(&mut substate);

            for (i, inner_index) in self.state_map.iter().enumerate() {
                state[outer_index | inner_index] = substate[i];
            }

            outer_index = ((outer_index | self.qubits_mask) + 1) & !self.qubits_mask;
        }
    }
}

/// Represents a quantum circuit as a sequence of gates
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Circuit {
    qubit_count: Option<u8>,
    gates: Vec<MappedGate>,
}

impl Circuit {
    /// Creates a new, empty circuit for a specified number of qubits
    pub fn new(qubit_count: u8) -> Self {
        assert!(
            qubit_count <= MAX_QUBITS,
            "Circuit can't have more than {MAX_QUBITS} qubits",
        );
        Self {
            qubit_count: Some(qubit_count),
            gates: Vec::new(),
        }
    }

    /// Adds a gate to the circuit, specifying the qubits it acts upon.
    ///
    /// The order of qubit indices matters, especially for controlled gates,
    /// following the convention `[control_1, ..., control_n, target_1, ...]`.
    pub fn add_gate<I: QubitIndices>(
        &mut self,
        gate: Gate,
        qubit_indices: I,
    ) -> Result<(), QubitError> {
        self.gates
            .push(MappedGate::new(gate, qubit_indices, self.qubit_count)?);

        Ok(())
    }

    /// Creates a circuit from a single gate and its target qubits.
    ///
    /// This is useful for treating a single gate operation as a circuit.
    pub fn from_gate<I: QubitIndices>(gate: Gate, qubit_indices: I) -> Result<Self, QubitError> {
        Ok(Self {
            qubit_count: None,
            gates: vec![MappedGate::new(gate, qubit_indices, None)?],
        })
    }

    /// Consumes the circuit and converts it into a single, composite `Gate`.
    ///
    /// This allows for hierarchical circuits, where a complex circuit can be used
    /// as a single gate within another, larger circuit.
    pub fn into_gate(self) -> Gate {
        Gate::circuit(self)
    }

    /// Creates a new circuit that is the inverse (adjoint) of this circuit
    pub fn inverse(&self) -> Self {
        let inverted_gates = self
            .gates
            .iter()
            // Reverse the order of the gates
            .rev()
            // Take the inverse of each gate (the mapping to qubits remains the same)
            .map(|mapped_gate| MappedGate {
                gate: mapped_gate.gate.inverse(),
                state_map: mapped_gate.state_map.clone(),
                qubits_mask: mapped_gate.qubits_mask,
            })
            .collect();

        Self {
            qubit_count: self.qubit_count,
            gates: inverted_gates,
        }
    }

    /// Applies the entire sequence of gates in the circuit to the given state vector.
    pub fn apply(&self, state: &mut [Complex64]) {
        for gate in &self.gates {
            gate.apply(state)
        }
    }
}

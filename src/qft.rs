//! Provides a builder function for creating a Quantum Fourier Transform (QFT) circuit.

use super::{
    circuit::{Circuit, QubitError},
    gate::Gate,
};

/// Builds a Quantum Fourier Transform (QFT) circuit
pub fn build_qft_circuit(n_qubits: u8) -> Result<Circuit, QubitError> {
    build_qft_circuit_custom(n_qubits, true)
}

/// Builds a QFT circuit with explicit control over the final SWAP gates
pub fn build_qft_circuit_custom(n_qubits: u8, do_swaps: bool) -> Result<Circuit, QubitError> {
    let mut circuit = Circuit::new(n_qubits);

    // Apply Hadamard and Controlled Phase gates
    for i in (0..n_qubits).rev() {
        circuit.add_gate(Gate::hadamard(), [i])?;
        // Apply Controlled Phase gates to qubits j > i
        for j in 0..i {
            let phase_fraction = 1f64 / f64::from(1 << (i - j + 1));
            circuit.add_gate(Gate::phase_fraction(phase_fraction).control(), [i, j])?;
        }
    }

    if do_swaps {
        for i in 0..n_qubits / 2 {
            circuit.add_gate(Gate::swap(), [i, n_qubits - i - 1])?;
        }
    }

    Ok(circuit)
}

#[cfg(test)]
mod tests {
    use crate::{
        gate::tests::{run_gate_tests, run_parameterized_gate_tests},
        tests::vectors,
    };

    use super::*;

    #[test]
    fn qft_circuit() {
        run_parameterized_gate_tests(
            "QFT",
            |test_vector| {
                let n_qubits = test_vector.initial_state.len().ilog2() as u8;
                build_qft_circuit(n_qubits).unwrap().into_gate()
            },
            &*vectors::QFT_TESTS,
        );
    }

    #[test]
    fn cqft_circuit() {
        run_gate_tests(
            "CQFT",
            build_qft_circuit(2).unwrap().into_gate().control(),
            &*vectors::CQFT_TESTS,
        );
    }

    #[test]
    fn ccqft_circuit() {
        run_gate_tests(
            "CCQFT1",
            build_qft_circuit(2).unwrap().into_gate().multi_control(2),
            &*vectors::CCQFT1_TESTS,
        );

        run_gate_tests(
            "CCQFT2",
            build_qft_circuit(2).unwrap().into_gate().multi_control(2),
            &*vectors::CCQFT2_TESTS,
        );
    }
}

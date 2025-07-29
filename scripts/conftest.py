# In scripts/conftest.py

import pytest # <-- Make sure to import pytest
from qiskit import QuantumCircuit
from qiskit.quantum_info import Statevector
import numpy as np

def run_gate_test(gate_builder, test_vector):
    """
    The core test runner logic. Encapsulates the boilerplate for a single test case.
    """
    n_qubits = (len(test_vector['initial_state'])).bit_length() - 1

    qc = QuantumCircuit(n_qubits)
    qc.initialize(test_vector['initial_state'], qc.qubits)
    
    gate_builder(qc, test_vector)
    
    final_state = Statevector.from_instruction(qc)
    
    # --- THIS IS THE MODIFIED PART ---
    try:
        assert_statevectors_close(final_state, test_vector)
    except AssertionError as e:
        # Use pytest.fail for clean output without the verbose traceback
        pytest.fail(str(e), pytrace=False)


def assert_statevectors_close(actual_state, expected_vector, tol=1e-6):
    """Asserts that two statevectors are equal within a tolerance."""
    actual_data = actual_state.data.flatten()
    expected_data = np.array(expected_vector['expected_state'], dtype=complex).flatten()

    def clean_vec(vec, tol):
        """Clean up small floating point artifacts for clearer failure messages."""
        return np.array([
            complex(0 if abs(amp.real) < tol else amp.real,
                    0 if abs(amp.imag) < tol else amp.imag)
            for amp in vec
        ])

    if not np.allclose(actual_data, expected_data, atol=tol):
        msg = (
            f"Statevectors do not match!\n"
            f"  Description: {expected_vector.get('description', 'N/A')}\n"
            f"  Actual:   {clean_vec(actual_data, tol)}\n"
            f"  Expected: {clean_vec(expected_data, tol)}"
        )
        raise AssertionError(msg)

@pytest.fixture
def test_runner():
    """A pytest fixture that provides the test runner function to tests."""
    return run_gate_test

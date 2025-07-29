import math
import pytest

from qiskit.circuit.library import QFTGate as QiskitQFT
from qft import QFT as LocalQFT
import vectors

# --- Standard Gate Tests ---

@pytest.mark.parametrize("test_vector", vectors.HADAMARD_TESTS)
def test_hadamard(test_runner, test_vector):
    gate_builder = lambda qc, v: qc.h(*v['qubits'])
    test_runner(gate_builder, test_vector)

@pytest.mark.parametrize("test_vector", vectors.NOT_TESTS)
def test_not(test_runner, test_vector):
    gate_builder = lambda qc, v: qc.x(*v['qubits'])
    test_runner(gate_builder, test_vector)

@pytest.mark.parametrize("test_vector", vectors.PHASE_TESTS)
def test_phase(test_runner, test_vector):
    gate_builder = lambda qc, v: qc.p(2 * math.pi * v['args']['fraction'], *v['qubits'])
    test_runner(gate_builder, test_vector)

@pytest.mark.parametrize("test_vector", vectors.SWAP_TESTS)
def test_swap(test_runner, test_vector):
    gate_builder = lambda qc, v: qc.swap(*v['qubits'])
    test_runner(gate_builder, test_vector)

@pytest.mark.parametrize("test_vector", vectors.CNOT_TESTS)
def test_cnot(test_runner, test_vector):
    gate_builder = lambda qc, v: qc.cx(*v['qubits'])
    test_runner(gate_builder, test_vector)

@pytest.mark.parametrize("test_vector", vectors.TOFFOLI_TESTS)
def test_toffoli(test_runner, test_vector):
    gate_builder = lambda qc, v: qc.ccx(*v['qubits'])
    test_runner(gate_builder, test_vector)

@pytest.mark.parametrize("test_vector", vectors.FREDKIN_TESTS)
def test_fredkin(test_runner, test_vector):
    gate_builder = lambda qc, v: qc.cswap(*v['qubits'])
    test_runner(gate_builder, test_vector)

# --- QFT Testing ---

@pytest.fixture(params=[QiskitQFT, LocalQFT], ids=["Qiskit_QFT", "Local_QFT"])
def qft_implementation(request):
    """Fixture to test both Qiskit's and the local QFT implementation."""
    return request.param

all_qft_tests = (
    vectors.QFT_TESTS +
    vectors.CQFT_TESTS +
    vectors.CCQFT1_TESTS +
    vectors.CCQFT2_TESTS
)

@pytest.mark.parametrize("test_vector", all_qft_tests)
def test_qft(test_runner, qft_implementation, test_vector):
    def gate_builder(qc, v):
        num_controls = v.get('num_controls')
        n_qft_qubits = qc.num_qubits if num_controls is None else qc.num_qubits - num_controls
        
        # The standard QiskitQFT and our LocalQFT both return Gate objects
        qft_gate = qft_implementation(num_qubits=n_qft_qubits)
        
        if num_controls is not None:
            qft_gate = qft_gate.control(num_controls)
        
        qc.append(qft_gate, v['qubits'])

    test_runner(gate_builder, test_vector)

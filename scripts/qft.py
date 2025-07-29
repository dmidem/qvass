from qiskit import QuantumCircuit
import numpy as np

def QFT(num_qubits: int,
             inverse: bool = False,
             do_swaps: bool = True):
    """
    Return an n-qubit QFT (or inverse QFT) gate that matches the
    textbook/Qiskit definition (little-endian; bit-reversal built in).

    Parameters
    ----------
    num_qubits : int
    inverse : bool, optional
        If True, build the inverse transform (default: False).
    do_swaps : bool, optional
        Insert the final bit-reversal SWAP network (default: True).
    """
    qc = QuantumCircuit(num_qubits, name="qft")

    # *** 1. phase-ladder from MSB → LSB ***
    for i in reversed(range(num_qubits)):         # i = n-1 … 0
        qc.h(i)
        for j in range(i):                        # j = 0 … i-1
            angle = np.pi / (2 ** (i - j))        # π / 2^{distance}
            qc.cp(angle, i, j)                    # control=i target=j

    # *** 2. optional bit-reversal SWAPs ***
    if do_swaps:
        for i in range(num_qubits // 2):
            qc.swap(i, num_qubits - i - 1)

    # *** 3. inverse = adjoint of the forward ***
    if inverse:
        qc = qc.inverse() # exact adjoint
        qc.name = "iqft"

    return qc #.to_gate(label=qc.name)


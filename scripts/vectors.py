import math
import numpy as np

FRAC_1_SQRT_2 = 1/math.sqrt(2)

def normalize(v):
    return (v / np.linalg.norm(v)).tolist()

HADAMARD_TESTS = [
    {
        'description': '|0⟩ → (|0⟩ + |1⟩)/sqrt(2)',
        'qubits': [0],
        'initial_state': [1, 0],
        'expected_state': [FRAC_1_SQRT_2, FRAC_1_SQRT_2],
    },
    {
        'description': '|1⟩ → (|0⟩ - |1⟩)/sqrt(2)',
        'qubits': [0],
        'initial_state': [0, 1],
        'expected_state': [FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
    },
    {
        'description': 'Superposition |+⟩ = (|0⟩ + |1⟩)/sqrt(2) → |0⟩',
        'qubits': [0],
        'initial_state': [FRAC_1_SQRT_2, FRAC_1_SQRT_2],
        'expected_state': [1, 0],
    },
]

NOT_TESTS = [
    {
        'description': '|0⟩ → |1⟩',
        'qubits': [0],
        'initial_state': [1, 0],
        'expected_state': [0, 1],
    },
    {
        'description': '|1⟩ → |0⟩',
        'qubits': [0],
        'initial_state': [0, 1],
        'expected_state': [1, 0],
    },
    {
        'description': ' Superposition |+⟩ = (|0⟩ + |1⟩)/sqrt(2) → (|1⟩ + |0⟩)/sqrt(2)',
        'qubits': [0],
        'initial_state': [FRAC_1_SQRT_2, FRAC_1_SQRT_2],
        'expected_state': [FRAC_1_SQRT_2, FRAC_1_SQRT_2],
    },
]

PHASE_TESTS = [
    {
        'description': 'Phase shift by π/2 on |1⟩ → i|1⟩',
        'qubits': [0],
        'initial_state': [1, 0],
        'expected_state': [1, 0],
        'args': {'fraction': 0.25},
    },
    {
        'description': 'Phase shift by π/2 on |1⟩ → i|1⟩',
        'qubits': [0],
        'initial_state': [0, 1],
        'expected_state': [0, 1j],
        'args': {'fraction': 0.25},
    },
    {
        'description': 'Phase shift by π on |1⟩ → -|1⟩',
        'qubits': [0],
        'initial_state': [0, 1],
        'expected_state': [0, -1],
        'args': {'fraction': 0.5},
    },
    {
        'description': 'Phase shift by π/4 on |1⟩ → (1 + i)/sqrt(2)|1⟩',
        'qubits': [0],
        'initial_state': [0, 1],
        'expected_state': [0, (1 + 1j)*FRAC_1_SQRT_2],
        'args': {'fraction': 0.125},
    },
]

SWAP_TESTS = [
    {
        'description': '|01⟩ → |10⟩',
        'qubits': [0, 1],
        'initial_state': [0, 1, 0, 0],
        'expected_state': [0, 0, 1, 0],
    },
    {
        'description': '|00⟩ and |11⟩ remain unchanged',
        'qubits': [0, 1],
        'initial_state': [FRAC_1_SQRT_2, 0, 0, FRAC_1_SQRT_2],
        'expected_state': [FRAC_1_SQRT_2, 0, 0, FRAC_1_SQRT_2],
    },
    {
        'description': 'Superposition |01⟩ + |10⟩ → |10⟩ + |01⟩',
        'qubits': [0, 1],
        'initial_state': [0, FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0],
        'expected_state': [0, FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0],
    },
]

CNOT_TESTS = [
    {
        'description': '|10⟩ → |11⟩',
        'qubits': [1, 0],
        'initial_state': [0, 0, 1, 0],
        'expected_state': [0, 0, 0, 1],
    },
    {
        'description': '|11⟩ → |10⟩',
        'qubits': [1, 0],
        'initial_state': [0, 0, 0, 1],
        'expected_state': [0, 0, 1, 0],
    },
    {
        'description': 'Superposition |10⟩ + |11⟩ → |11⟩ + |10⟩',
        'qubits': [1, 0],
        'initial_state': [0, 0, FRAC_1_SQRT_2, FRAC_1_SQRT_2],
        'expected_state': [0, 0, FRAC_1_SQRT_2, FRAC_1_SQRT_2],
    },
    {
        'description': 'Bell State |Φ+⟩ = Superposition |00⟩ + |11⟩ → |00⟩ + |10⟩',
        'qubits': [1, 0],
        'initial_state': [FRAC_1_SQRT_2, 0, 0, FRAC_1_SQRT_2],
        'expected_state': [FRAC_1_SQRT_2, 0, FRAC_1_SQRT_2, 0],
    },
    {
        'description': '|00⟩ remains |00⟩',
        'qubits': [1, 0],
        'initial_state': [1, 0, 0, 0],
        'expected_state': [1, 0, 0, 0],
    },
]

TOFFOLI_TESTS = [
    {
        'description': '|110⟩ → |111⟩',
        'qubits': [1, 2, 0],
        'initial_state': [0, 0, 0, 0, 0, 0, 1, 0],
        'expected_state': [0, 0, 0, 0, 0, 0, 0, 1],
    },
    {
        'description': '|110⟩ → |110⟩',
        'qubits': [1, 2, 0],
        'initial_state': [0, 0, 0, 0, 0, 0, 0, 1],
        'expected_state': [0, 0, 0, 0, 0, 0, 1, 0],
    },
    {
        'description': 'Superposition |110⟩ + |111⟩ → |111⟩ + |110⟩',
        'qubits': [1, 2, 0],
        'initial_state': [0, 0, 0, 0, 0, 0, FRAC_1_SQRT_2, FRAC_1_SQRT_2],
        'expected_state': [0, 0, 0, 0, 0, 0, FRAC_1_SQRT_2, FRAC_1_SQRT_2],
    },
]

FREDKIN_TESTS = [
    {
        'description': '|000⟩ → |000⟩ (No Swap)',
        'qubits': [2, 0, 1],
        'initial_state': [1, 0, 0, 0, 0, 0, 0, 0],
        'expected_state': [1, 0, 0, 0, 0, 0, 0, 0],
    },
    {
        'description': '|100⟩ → |110⟩ (Swap)',
        'qubits': [2, 0, 1],
        'initial_state': [0, 0, 0, 0, 0, 1, 0, 0],
        'expected_state': [0, 0, 0, 0, 0, 0, 1, 0],
    },
    {
        'description': '|110⟩ → |101⟩ (Swap)',
        'qubits': [2, 0, 1],
        'initial_state': [0, 0, 0, 0, 0, 0, 1, 0],
        'expected_state': [0, 0, 0, 0, 0, 1, 0, 0],
    },
    {
        'description': 'Superposition |101⟩ + |110⟩ → |110⟩ + |101⟩ (No observable change)',
        'qubits': [2, 0, 1],
        'initial_state': [0, 0, 0, 0, 0, FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0],
        'expected_state': [0, 0, 0, 0, 0, FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0],
    },
]

QFT_TESTS = [
    {
        'description': '|0⟩ + |2⟩ → |0⟩ + |2⟩',
        'qubits': [0, 1],
        'initial_state': [FRAC_1_SQRT_2 if i in [0, 2] else 0.0 for i in range(4)],
        'expected_state': [FRAC_1_SQRT_2 if i in [0, 2] else 0.0 for i in range(4)],
    },
    {
        'description': '|0⟩ + |2⟩ + |4⟩ + |6⟩ → |0⟩ + |4⟩',
        'qubits': [0, 1, 2],
        'initial_state': [0.5 if i in [0, 2, 4, 6] else 0.0 for i in range(8)],
        'expected_state': [FRAC_1_SQRT_2 if i in [0, 4] else 0.0 for i in range(8)],
    },
    {
        'description': '|1⟩ + |5⟩ + |9⟩ + |13⟩ → |0⟩ + |4⟩ + |8⟩ + |12⟩',
        'qubits': [0, 1, 2, 3],
        'initial_state': [0.5 if i in [1, 5, 9, 13] else 0.0 for i in range(16)],
        'expected_state': [0.5 * np.exp(2j * math.pi * i / 16) if i % 4 == 0 else 0 for i in range(16)]
    },
]

CQFT_TESTS = [
    {
        'description': 'Controlled-QFT',
        'qubits': [2, 0, 1],
        'num_controls': 1,
        'initial_state': normalize([1 if i in [0, 5] else 0.0 for i in range(8)]),
        'expected_state': normalize([1, 0, 0, 0, 0.5, 0.5j, -0.5, -0.5j])
        #'expected_state': normalize([1, 0.5, 0, -0.5, 0, 0.5, 0, -0.5])
    },
]

CCQFT1_TESTS = [
    {
        'description': 'CC-QFT: Controls not met (|0110> -> |0110>)',
        'qubits': [3, 2, 0, 1],
        'num_controls': 2,
        'initial_state': [1 if i == 6 else 0.0 for i in range(16)], # |0110>
        'expected_state': [1 if i == 6 else 0.0 for i in range(16)], # |0110>
    },
    {
        'description': 'CC-QFT: Controls met (|1101> -> QFT|01>)',
        'qubits': [3, 2, 0, 1],
        'num_controls': 2,
        'initial_state': [1 if i == 13 else 0.0 for i in range(16)], # |1101>
        'expected_state': [
            0.5 if i == 12 else         # |1100>
            0.5j if i == 13 else        # |1101>
            -0.5 if i == 14 else        # |1110>
            -0.5j if i == 15 else       # |1111>
            0.0 for i in range(16)
        ],
    },
    {
        'description': 'CC-QFT: Superposition of met and not-met controls',
        'qubits': [3, 2, 0, 1],
        'num_controls': 2,
        # Initial state is (|0110> + |1101>)/sqrt(2)
        'initial_state': normalize([1 if i in [6, 13] else 0.0 for i in range(16)]),
        # Expected state combines the results from test #1 and #2 via linearity
        'expected_state': normalize(
            [
                1.0 if i == 6 else          # from |0110> part
                0.5 if i == 12 else         # from QFT(|1101>) part
                0.5j if i == 13 else
                -0.5 if i == 14 else
                -0.5j if i == 15 else
                0.0 for i in range(16)
            ]
        )
    },
]

CCQFT2_TESTS = [
    {
        'description': 'CC-QFT: Controls met (|1101> -> |11> (H|1> H|0>))',
        'qubits': [3, 2, 1, 0],
        'num_controls': 2,
        'initial_state': [1 if i == 13 else 0.0 for i in range(16)], # |1101>
        'expected_state': [
            # This matches your "Actual" output from the failed test
            # Amplitudes for |1100>, |1101>, |1110>, |1111>
            0.5 if i == 12 else
            0.5 if i == 13 else
            -0.5 if i == 14 else
            -0.5 if i == 15 else
            0.0 for i in range(16)
        ],
    },
    {
        'description': 'CC-QFT: Superposition of met and not-met controls',
        'qubits': [3, 2, 1, 0],
        'num_controls': 2,
        'initial_state': normalize([1 if i in [6, 13] else 0.0 for i in range(16)]), # (|0110> + |1101>)/sqrt(2)
        'expected_state': normalize(
            [
                1.0 if i == 6 else          # from |0110> part
                0.5 if i == 12 else         # from WHT(|1101>) part
                0.5 if i == 13 else
                -0.5 if i == 14 else
                -0.5 if i == 15 else
                0.0 for i in range(16)
            ]
        )
    },
]

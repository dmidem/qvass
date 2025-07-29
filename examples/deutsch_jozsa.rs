use rand::{rngs::SmallRng, SeedableRng};

use qvass::{Gate, QuantumSimulator, QubitError};

/// This example demonstrates the Deutsch-Jozsa algorithm, one of the first
/// quantum algorithms to show an advantage over classical methods.
///
/// The problem: Given a function `f(x)` that takes N bits and returns 1 bit,
/// we are promised that `f` is either "constant" (returns the same value for
/// all inputs) or "balanced" (returns 0 for half the inputs and 1 for the
/// other half). The goal is to determine which it is.
///
/// Classically, this requires at least two queries. Quantumly, we can do it with one.
fn main() -> Result<(), QubitError> {
    // We'll solve for a 2-bit function, so we need N=2 input qubits
    // and 1 output qubit for the oracle.
    const N_INPUT_QUBITS: u8 = 2;
    let output_qubit_index = N_INPUT_QUBITS;

    // --- Case 1: A "Constant" Oracle ---
    // This oracle represents a function that always returns 1, i.e., f(x) = 1.
    // The oracle U_f maps |x>|y> to |x>|y ⊕ f(x)>.
    // For f(x) = 1, this is just a bit-flip (X gate) on the output qubit.
    println!("--- Testing a CONSTANT function: f(x) = 1 ---");
    let constant_oracle = Gate::not();
    let constant_result = run_deutsch_jozsa(N_INPUT_QUBITS, constant_oracle, [output_qubit_index])?;

    // For a constant function, the measurement of the input qubits will always be |0...0>.
    println!(
        "Measured state of input qubits: |{:0width$b}>",
        constant_result,
        width = N_INPUT_QUBITS as usize
    );
    println!("Result: The function is CONSTANT.");
    assert_eq!(constant_result, 0);

    println!("\n------------------------------------------------\n");

    // --- Case 2: A "Balanced" Oracle ---
    // This oracle represents a function f(x) = x_0 (the first input bit).
    // This function is balanced. The oracle for it is a CNOT gate, where
    // the control is the input qubit x_0 and the target is the output qubit.
    println!("--- Testing a BALANCED function: f(x) = x_0 ---");
    let balanced_oracle = Gate::cnot();
    let balanced_result =
        run_deutsch_jozsa(N_INPUT_QUBITS, balanced_oracle, [0, output_qubit_index])?;

    // For a balanced function, the measurement of the input qubits will never be |0...0>.
    println!(
        "Measured state of input qubits: |{:0width$b}>",
        balanced_result,
        width = N_INPUT_QUBITS as usize
    );
    println!("Result: The function is BALANCED.");
    assert_ne!(balanced_result, 0);

    Ok(())
}

/// Executes the core Deutsch-Jozsa algorithm for a given oracle.
fn run_deutsch_jozsa<const N: usize>(
    n_input_qubits: u8,
    oracle_gate: Gate,
    oracle_qubits: [u8; N],
) -> Result<usize, QubitError> {
    let total_qubits = n_input_qubits + 1;
    let output_qubit_index = n_input_qubits;
    let input_qubit_indices: Vec<u8> = (0..n_input_qubits).collect();

    let mut sim = QuantumSimulator::new(total_qubits);

    // 1. Initialize state: input qubits to |0...0⟩, output qubit to |1⟩.
    sim.init_state(0);
    sim.add_gate(Gate::not(), [output_qubit_index])?;

    // 2. Apply Hadamard to all qubits to create superposition.
    for i in 0..total_qubits {
        sim.add_gate(Gate::hadamard(), [i])?;
    }

    // 3. Apply the oracle gate, which encodes the function f(x).
    sim.add_gate(oracle_gate, oracle_qubits)?;

    // 4. Apply Hadamard to the input qubits only.
    for &i in &input_qubit_indices {
        sim.add_gate(Gate::hadamard(), [i])?;
    }

    // 5. Create a seeded RNG for reproducible measurements.
    // For a real simulation, you might seed this from the system time.
    let mut rng = SmallRng::seed_from_u64(42);

    // 5. Run the circuit and measure the state of the *input* qubits.
    sim.run();
    let full_measurement = sim.measure(&mut rng);

    // The result is determined by the state of the first N qubits.
    // We mask away the output qubit's state.
    let final_result = full_measurement & ((1 << n_input_qubits) - 1);
    Ok(final_result)
}

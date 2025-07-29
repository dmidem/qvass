use rand::{rngs::SmallRng, SeedableRng};

use qvass::{qft, QuantumSimulator, QubitError};

/// This example demonstrates the Quantum Fourier Transform (QFT).
///
/// It performs the following steps:
/// 1. Creates a 3-qubit QFT circuit using the `build_qft_circuit` helper.
/// 2. Initializes a simulator in the |001⟩ basis state.
/// 3. Applies the QFT circuit to the state.
/// 4. Prints the final state vector, showing the complex amplitudes.
/// 5. Runs the simulation many times to build a histogram of measurement outcomes,
///    demonstrating that the QFT creates a uniform superposition.
fn main() -> Result<(), QubitError> {
    const NUM_QUBITS: u8 = 3;

    // 1. Build the QFT circuit for 3 qubits.
    // We can then treat this entire circuit as a single, reusable gate.
    let qft_gate = qft::build_qft_circuit(NUM_QUBITS)?.into_gate();

    // 2. Initialize the simulator and add the QFT gate.
    let mut sim = QuantumSimulator::new(NUM_QUBITS);
    sim.add_gate(qft_gate, [0, 1, 2])?;

    // 3. Prepare the initial state |001⟩.
    let initial_state_index = 1;
    sim.init_state(initial_state_index);

    println!(
        "Initial state is |{:0width$b}>",
        initial_state_index,
        width = NUM_QUBITS as usize
    );

    // 4. Run the simulation once to see the final state vector.
    sim.run();
    println!("\nState vector after applying QFT:");
    println!("{}", sim.state());

    // 5. Create a seeded RNG for reproducible measurements.
    // For a real simulation, you might seed this from the system time.
    let mut rng = SmallRng::seed_from_u64(42);

    // 6. Run the simulation many times to see the probability distribution.
    // The QFT of a basis state results in a uniform superposition, so all
    // outcomes should be roughly equally likely.
    let n_iterations = 4096;
    let mut histogram = vec![0; sim.state().as_ref().len()];

    println!("\nBuilding histogram from {n_iterations} measurements...");
    for _ in 0..n_iterations {
        // Reset the state and run the simulation for each measurement.
        sim.init_state(initial_state_index);
        sim.run();
        let outcome = sim.measure(&mut rng);
        histogram[outcome] += 1;
    }

    println!("\nMeasurement Histogram:");
    for (i, &count) in histogram.iter().enumerate() {
        println!(
            "  |{:0width$b}> : {}",
            i,
            count,
            width = NUM_QUBITS as usize
        );
    }
    println!(
        "\nNote: Each outcome has a probability of ~1/8, so counts should be around {}.",
        n_iterations / 8
    );

    Ok(())
}

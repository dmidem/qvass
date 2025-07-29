//! Implements a comprehensive demonstration of Grover's search algorithm.
//!
//! This example is split into three parts:
//! 1. A detailed, step-by-step walkthrough of the first iteration, showing the
//!    state vector after each major operation (Superposition, Oracle, Diffuser).
//! 2. An execution of the full algorithm for the optimal number of iterations,
//!    showing the final, maximally amplified state vector.
//! 3. A statistical analysis over 1000 runs to calculate and verify the high
//!    success rate of the algorithm.

use std::f64::consts::PI;

use rand::{rngs::SmallRng, SeedableRng};

use qvass::{Circuit, Gate, QuantumSimulator, QubitError};

/// Builds the Oracle for Grover's algorithm.
/// The Oracle "marks" the winning state by flipping its phase.
fn build_oracle(n_qubits: u8, winning_state: usize) -> Result<Circuit, QubitError> {
    let mut oracle_circuit = Circuit::new(n_qubits);
    let all_qubits: Vec<u8> = (0..n_qubits).collect();

    // To mark an arbitrary state, we surround a multi-controlled Z gate
    // with X gates on the qubits that are 0 in the winning state.
    for i in 0..n_qubits {
        if (winning_state >> i) & 1 == 0 {
            oracle_circuit.add_gate(Gate::not(), [i])?;
        }
    }
    let controlled_z = Gate::phase_radians(PI).multi_control(n_qubits - 1);
    oracle_circuit.add_gate(controlled_z, all_qubits.clone())?;
    for i in 0..n_qubits {
        if (winning_state >> i) & 1 == 0 {
            oracle_circuit.add_gate(Gate::not(), [i])?;
        }
    }
    Ok(oracle_circuit)
}

/// Builds the Diffuser (amplitude amplification) operator.
/// This circuit amplifies the amplitude of the marked state.
fn build_diffuser(n_qubits: u8) -> Result<Circuit, QubitError> {
    let mut diffuser_circuit = Circuit::new(n_qubits);
    let all_qubits: Vec<u8> = (0..n_qubits).collect();
    let zero_oracle = build_oracle(n_qubits, 0)?;

    for i in 0..n_qubits {
        diffuser_circuit.add_gate(Gate::hadamard(), [i])?;
    }
    diffuser_circuit.add_gate(zero_oracle.into_gate(), all_qubits.clone())?;
    for i in 0..n_qubits {
        diffuser_circuit.add_gate(Gate::hadamard(), [i])?;
    }
    Ok(diffuser_circuit)
}

fn main() -> Result<(), QubitError> {
    const N_QUBITS: u8 = 4;
    const WINNING_STATE: usize = 5; // We are searching for the state |101⟩

    // --- Algorithm Setup ---
    let n_items = 1 << N_QUBITS;
    let num_iterations = (PI / 4.0 * (n_items as f64).sqrt()).round() as usize;

    println!("Grover's Search for {N_QUBITS} qubits ({n_items} items)");
    println!("Searching for marked state: |{WINNING_STATE}>");
    println!("Optimal number of iterations: {num_iterations}");

    // --- Part 1: Detailed Walkthrough of the First Iteration ---
    println!("\n=== Part 1: Step-by-Step Walkthrough ===");

    let mut sim = QuantumSimulator::new(N_QUBITS);
    let all_qubits: Vec<u8> = (0..N_QUBITS).collect();

    // Build the main algorithm components once.
    let oracle = build_oracle(N_QUBITS, WINNING_STATE)?;
    let diffuser = build_diffuser(N_QUBITS)?;

    // Step 1: Create a uniform superposition.
    for i in 0..N_QUBITS {
        sim.add_gate(Gate::hadamard(), [i])?;
    }
    sim.init_state(0);
    sim.run(); // Applies the Hadamard gates.
    println!("\n--- 1. State after Initial Superposition ---");
    println!("{}", sim.state());

    // Step 2: Apply the Oracle.
    sim.add_gate(oracle.clone().into_gate(), all_qubits.clone())?;
    sim.init_state(0);
    sim.run();
    println!("\n--- 2. State after Oracle (amplitude of |101> is flipped) ---");
    println!("{}", sim.state());

    // Step 3: Apply the Diffuser.
    sim.add_gate(diffuser.clone().into_gate(), all_qubits.clone())?;
    sim.init_state(0);
    sim.run();
    println!("\n--- 3. State after 1st Grover Iteration (amplitude of |101> is amplified) ---");
    println!("{}", sim.state());

    // --- Part 2: State After All Iterations ---
    println!("\n=== Part 2: State After Full Amplification ===");

    // Add the remaining Grover iterations to the circuit.
    if num_iterations > 1 {
        for _ in 1..num_iterations {
            sim.add_gate(oracle.clone().into_gate(), all_qubits.clone())?;
            sim.add_gate(diffuser.clone().into_gate(), all_qubits.clone())?;
        }
    }

    // Run the complete circuit from the start.
    sim.init_state(0);
    sim.run();
    println!("\n--- State after {num_iterations} Grover Iterations ---");
    println!("{}", sim.state());

    // --- Part 3: Statistical Analysis ---
    println!("\n=== Part 3: Statistical Analysis ===");
    println!("Running simulation 1000 times to find success rate...");

    // For a real simulation, you might seed this from the system time.
    let mut rng = SmallRng::seed_from_u64(42);

    let mut success_count = 0;
    let n_runs = 1000;

    for _ in 0..n_runs {
        // The simulator `sim` now contains the complete, optimal circuit.
        // We just need to re-initialize and re-run for each trial.
        sim.init_state(0);
        sim.run();
        let outcome = sim.measure(&mut rng);

        if outcome == WINNING_STATE {
            success_count += 1;
        }
    }

    let success_rate = (success_count as f64 / n_runs as f64) * 100.0;
    println!("\nFound the correct state {success_count} times out of {n_runs} runs.");
    println!("Success rate: {success_rate:.1}%");
    assert!(success_rate > 85.0, "Success rate should be very high.");

    Ok(())
}

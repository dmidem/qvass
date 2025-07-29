//! A Rust library for building and running quantum circuit simulations.
//!
//! This crate provides the tools to build, simulate, and measure quantum circuits.
//! The primary entry point is the [`QuantumSimulator`], which manages the state
//! and execution of a [`Circuit`]. Circuits themselves are constructed from the
//! fundamental building blocks defined in the [`Gate`] enum.
//!
//! The simulator's behavior is rigorously tested against Qiskit to ensure correctness.
//!
//! ## Getting Started
//!
//! Here is a quick example that creates a 3-qubit GHZ state (`(|000⟩ + |111⟩)/√2`),
//! a classic example of quantum entanglement.
//!
//! ```rust
//! use rand::{rngs::SmallRng,  SeedableRng};
//!
//! use qvass::{QuantumSimulator, Gate, QubitError};
//!
//! fn main() -> Result<(), QubitError> {
//!     // 1. Create a simulator for a 3-qubit system.
//!     let mut sim = QuantumSimulator::new(3);
//!
//!     // 2. Build the circuit to create the GHZ state.
//!     sim.add_gate(Gate::hadamard(), [0])?;
//!     sim.add_gate(Gate::cnot(), [0, 1])?;
//!     sim.add_gate(Gate::cnot(), [0, 2])?;
//!
//!     // 3. Create a seeded RNG for reproducible measurements.
//!     // For a real simulation, you might seed this from the system time.
//!     let mut rng = SmallRng::seed_from_u64(123);
//!
//!     // 4. Start from the |000⟩ state, run the simulation, and measure.
//!     sim.init_state(0);
//!     sim.run();
//!     let outcome = sim.measure(&mut rng);
//!
//!     // After measurement, the state will be either |000⟩ (index 0)
//!     // or |111⟩ (index 7), with a 50/50 chance for each.
//!     println!("Measured state: |{}>", outcome);
//!     assert!(outcome == 0 || outcome == 7);
//!
//!     Ok(())
//! }
//! ```

#![no_std]

extern crate alloc;

#[cfg(test)]
extern crate std;

mod circuit;
mod gate;
mod simulator;

pub mod classical;

pub mod qft;

pub use circuit::{Circuit, QubitError};
pub use gate::Gate;
pub use simulator::QuantumSimulator;

#[cfg(test)]
mod tests;

// To run doc tests on examples from README.md and verify their correctness
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadMe;

//! # conservation-composer
//!
//! **Music composition governed by the spectral conservation laws of graph theory.**
//!
//! This crate converts graph topology into music by mapping the eigenvalue spectrum
//! of the graph Laplacian to musical parameters: pitch, rhythm, harmony, and dynamics.
//! The **Conservation Ratio** CR = λ₂/λₙ — the ratio of algebraic connectivity to
//! spectral radius — becomes the single control parameter that determines the
//! musical style: consonant (CR > 0.7), jazz (0.3 < CR < 0.7), or atonal (CR < 0.3).
//!
//! ## The Key Insight
//!
//! Graphs and music share a deep structural parallel. The eigenvalues of the graph
//! Laplacian encode connectivity: λ₁ = 0 is the "drone", λ₂ is the "tonic", and
//! higher eigenvalues are "overtones". The Fiedler vector (eigenvector of λ₂)
//! partitions the graph into two communities — these become the "strong" and "weak"
//! beats. The conservation ratio CR controls the tension between stability and
//! chaos: high CR produces consonant major-key music (everything is well-connected),
//! while low CR produces atonal experimental music (the graph has bottlenecks).
//!
//! This isn't arbitrary mapping — it's the **mathematical structure of the graph
//! singing its own topology**.
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────┐
//! │   Graph Topology      │    Nodes + edges (adjacency matrix)
//! │   (user-supplied)     │
//! └──────────┬───────────┘
//!            │ Build Laplacian L = D - A
//! ┌──────────▼───────────┐
//! │   Jacobi Eigenvalue   │    λ₁=0, λ₂=algebraic connectivity,
//! │   Decomposition       │    Fiedler vector partitions graph
//! └──────────┬───────────┘
//!            │
//! ┌──────────▼───────────┐
//! │   Conservation Ratio  │    CR = λ₂ / λₙ
//! │   CR → Style          │    high=major, mid=jazz, low=chromatic
//! └──────────┬───────────┘
//!            │
//!     ┌──────┼──────────┐
//!     │      │          │
//! ┌───▼──┐ ┌─▼───┐ ┌───▼────┐
//! │Pitch │ │Rhythm│ │Dynamics│
//! │λ → MIDI│ │Fiedler│ │eigenvalue│
//! │scale  │ │vector │ │amplitude │
//! └───┬──┘ └─┬───┘ └───┬────┘
//!     │      │         │
//! ┌───▼──────▼─────────▼──┐
//! │   Musical Score        │    Vec<NoteEvent> — pitch, time, duration, velocity
//! └────────────────────────┘
//! ```
//!
//! ## Quick Start
//!
//! ```rust
//! use conservation_composer::{Graph, MusicalParams, compose};
//!
//! // Build a 6-node cycle graph
//! let graph = Graph::cycle(6);
//!
//! // Compute the musical parameters
//! let params = compose(&graph);
//!
//! println!("CR = {:.4} → style: {}", params.cr, params.style_name());
//! println!("BPM: {}", params.bpm);
//! println!("Key: {}", params.key_name());
//! println!("Notes: {} voices", params.notes.len());
//! for note in &params.notes {
//!     println!("  {:3} | MIDI {:3} | {:.2} Hz | amp {:.2} | dur {:.2}",
//!              note.role, note.midi, note.freq, note.amplitude, note.duration_mult);
//! }
//! ```

mod eigen;
mod graph;
mod score;

pub use eigen::EigenDecomposition;
pub use graph::Graph;
pub use score::{compose, generate_score, MusicalNote, MusicalParams, NoteEvent, NoteRole};

/// Standard MIDI note number to frequency conversion (A4 = 440 Hz).
pub fn midi_to_freq(midi: u8) -> f64 {
    440.0 * 2.0_f64.powf((midi as f64 - 69.0) / 12.0)
}

/// Frequency to MIDI note number.
pub fn freq_to_midi(freq: f64) -> f64 {
    69.0 + 12.0 * (freq / 440.0).log2()
}

/// Note name from MIDI number (e.g., 60 → "C4").
pub fn midi_to_name(midi: u8) -> String {
    const NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let name = NAMES[(midi as usize) % 12];
    let octave = (midi as i32 / 12) - 1;
    format!("{}{}", name, octave)
}

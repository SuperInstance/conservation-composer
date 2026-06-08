//! # conservation-composer
//!
//! Algorithmic composition maximizing **spectral conservation**.
//!
//! A jazz ii-V-I progression is mathematically optimal because the voice-leading
//! distance between chords minimizes spectral energy loss. This crate generates
//! chord progressions whose transition matrix has eigenvalues matching conservation
//! constraints, using simulated annealing over a graph-Laplacian representation.
//!
//! ## Quick Start
//!
//! ```rust
//! use conservation_composer::{compose::ComposeStyle, style::quick_compose};
//!
//! let progression = quick_compose(ComposeStyle::Jazz, 4).unwrap();
//! for chord in &progression.chords {
//!     println!("{}", chord);
//! }
//! ```

pub mod annealer;
pub mod chord;
pub mod compose;
pub mod error;
pub mod laplacian;
pub mod progression;
pub mod spectral;
pub mod style;

// Re-export the main types at the crate root.
pub use annealer::{Annealer, ConservationConstraint};
pub use chord::{Chord, ChordQuality};
pub use compose::{ComposeStyle, Composer};
pub use error::ComposeError;
pub use laplacian::{build_laplacian, build_transition_matrix};
pub use progression::{ChordProgression, KeyMode, KeySignature};
pub use spectral::{algebraic_connectivity, eigenvalues, spectral_gap};

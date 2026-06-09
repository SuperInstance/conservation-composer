//! Musical score generation from graph spectral properties.
//!
//! Maps the eigenvalue spectrum of a graph Laplacian to musical parameters:
//! pitch (MIDI notes), rhythm (Fiedler vector), dynamics (eigenvalue magnitude),
//! and style (conservation ratio).

use crate::eigen::EigenDecomposition;
use crate::graph::Graph;
use crate::{midi_to_freq, midi_to_name};

/// The role a note plays in the spectral composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteRole {
    /// λ₁ = 0 → bass drone / pedal tone.
    Drone,
    /// λ₂ → tonal center / root.
    Root,
    /// λ₃–λ₅ → chord tones (harmonic backbone).
    Chord,
    /// λ₆+ → overtones / color tones.
    Overtone,
}

impl std::fmt::Display for NoteRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteRole::Drone => write!(f, "drone"),
            NoteRole::Root => write!(f, "root "),
            NoteRole::Chord => write!(f, "chord"),
            NoteRole::Overtone => write!(f, "over"),
        }
    }
}

/// A single note in the spectral composition.
#[derive(Debug, Clone)]
pub struct MusicalNote {
    /// MIDI note number (0-127).
    pub midi: u8,
    /// Frequency in Hz.
    pub freq: f64,
    /// Duration multiplier (relative to beat).
    pub duration_mult: f64,
    /// Amplitude (0.0-1.0).
    pub amplitude: f64,
    /// Eigenvalue index (which eigenvalue this note corresponds to).
    pub eigen_index: usize,
    /// Eigenvalue magnitude.
    pub eigen_value: f64,
    /// Role in the composition.
    pub role: NoteRole,
    /// Note name (e.g., "C4").
    pub name: String,
}

/// Musical parameters derived from graph spectral analysis.
#[derive(Debug, Clone)]
pub struct MusicalParams {
    /// Conservation ratio CR = λ₂/λₙ.
    pub cr: f64,
    /// Beats per minute, derived from CR.
    pub bpm: u32,
    /// Root MIDI note.
    pub root_midi: u8,
    /// Scale intervals (semitones from root).
    pub scale: Vec<u8>,
    /// Scale name.
    pub scale_name: String,
    /// Musical notes, one per eigenvalue.
    pub notes: Vec<MusicalNote>,
    /// Full eigendecomposition.
    pub decomp: EigenDecomposition,
}

impl MusicalParams {
    /// Human-readable style name based on CR.
    pub fn style_name(&self) -> &'static str {
        if self.cr > 0.7 {
            "Consonant / Major"
        } else if self.cr > 0.3 {
            "Jazz / Extended"
        } else {
            "Atonal / Experimental"
        }
    }

    /// Key and scale name (e.g., "C4 Major").
    pub fn key_name(&self) -> String {
        format!("{} {}", midi_to_name(self.root_midi), self.scale_name)
    }
}

/// Compose music from a graph's spectral properties.
///
/// This is the main entry point: give it a graph, and it returns a complete
/// set of musical parameters including individual notes mapped from eigenvalues.
///
/// # Algorithm
///
/// 1. Compute the Laplacian eigendecomposition
/// 2. Derive CR → style (scale, BPM)
/// 3. Map each eigenvalue to a MIDI note based on its position and magnitude
/// 4. Derive rhythm from the Fiedler vector (positive = long notes, negative = short)
/// 5. Derive dynamics from eigenvalue index (lower = louder, higher = quieter)
pub fn compose(graph: &Graph) -> MusicalParams {
    let lap = graph.laplacian();
    let decomp = EigenDecomposition::compute(&lap, graph.n);
    let cr = decomp.conservation_ratio();

    // CR → style mapping
    let (root_midi, scale, scale_name, bpm) = if cr > 0.7 {
        (
            60u8, // C4
            vec![0, 2, 4, 5, 7, 9, 11], // Major
            "Major".to_string(),
            100u32,
        )
    } else if cr > 0.3 {
        (
            60u8,
            vec![0, 2, 4, 6, 7, 9, 10, 11], // Melodic minor / jazz
            "Jazz".to_string(),
            130u32,
        )
    } else {
        (
            60u8,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], // Chromatic
            "Chromatic".to_string(),
            160u32,
        )
    };

    let fiedler = decomp.fiedler_vector();
    let max_eigen = decomp.eigenvalues.last().copied().unwrap_or(1.0).max(1e-10);

    let notes: Vec<MusicalNote> = decomp
        .eigenvalues
        .iter()
        .enumerate()
        .map(|(i, &lambda)| {
            let (midi, role) = if i == 0 {
                (root_midi.saturating_sub(12), NoteRole::Drone)
            } else if i == 1 {
                (root_midi, NoteRole::Root)
            } else if i <= 4 {
                let ratio = lambda / max_eigen;
                let idx = ((ratio * scale.len() as f64 * 2.0) as usize).min(scale.len() - 1);
                let mut m = root_midi + scale[idx];
                if i == 3 {
                    m += 4;
                }
                if i == 4 {
                    m += 7;
                }
                (m, NoteRole::Chord)
            } else {
                let ratio = lambda / max_eigen;
                let octave_shift = ((i - 5) / 3) as u8;
                let idx = ((ratio * scale.len() as f64) as usize) % scale.len();
                (root_midi + 12 + octave_shift * 12 + scale[idx], NoteRole::Overtone)
            };

            let fiedler_val = fiedler
                .and_then(|f| f.get(i).copied())
                .unwrap_or(0.0);
            let duration_mult = (1.0 + fiedler_val * 0.8).max(0.25);

            let amplitude = match role {
                NoteRole::Drone => 0.15,
                NoteRole::Root => 0.25,
                NoteRole::Chord => 0.12,
                NoteRole::Overtone => (0.08 / (1.0 + (i as f64 - 4.0) * 0.3)).max(0.03),
            };

            MusicalNote {
                midi,
                freq: midi_to_freq(midi),
                duration_mult,
                amplitude,
                eigen_index: i,
                eigen_value: lambda,
                role,
                name: midi_to_name(midi),
            }
        })
        .collect();

    MusicalParams {
        cr,
        bpm,
        root_midi,
        scale,
        scale_name,
        notes,
        decomp,
    }
}

/// Generate a sequence of note events for playback or MIDI export.
///
/// Given the musical parameters, produces a time-ordered sequence of note-on
/// events across `total_beats` beats. Uses the Fiedler vector for probabilistic
/// rhythm: positive Fiedler values increase the probability of a note playing.
#[derive(Debug, Clone)]
pub struct NoteEvent {
    /// Beat number (0-based).
    pub beat: u32,
    /// Note index in the params.notes array.
    pub note_index: usize,
    /// MIDI note number.
    pub midi: u8,
    /// Duration in beats.
    pub duration_beats: f64,
    /// Velocity (0.0-1.0).
    pub velocity: f64,
}

/// Generate a score (sequence of note events) from musical parameters.
pub fn generate_score(params: &MusicalParams, total_beats: u32) -> Vec<NoteEvent> {
    let fiedler = params.decomp.fiedler_vector();
    let mut events = Vec::new();

    for beat in 0..total_beats {
        for (ni, note) in params.notes.iter().enumerate() {
            // Drone plays on beats 0, 4, 8, 12, ...
            if matches!(note.role, NoteRole::Drone) && beat % 4 != 0 {
                continue;
            }

            // Probabilistic rhythm based on Fiedler vector
            if ni > 0 {
                let fiedler_val = fiedler
                    .and_then(|f| f.get(ni).copied())
                    .unwrap_or(0.0);
                let prob = 0.3 + 0.4 * (1.0 + fiedler_val) / 2.0;
                // Deterministic pseudo-random from beat+ni
                let hash = (beat as f64 * 127.1 + ni as f64 * 311.7).sin() * 43758.5453;
                let frac = hash - hash.floor();
                if frac > prob {
                    continue;
                }
            }

            events.push(NoteEvent {
                beat,
                note_index: ni,
                midi: note.midi,
                duration_beats: note.duration_mult,
                velocity: note.amplitude,
            });
        }
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_cycle() {
        let g = Graph::cycle(6);
        let params = compose(&g);
        assert!(params.cr > 0.0);
        assert!(params.cr < 1.0);
        assert_eq!(params.notes.len(), 6);
    }

    #[test]
    fn test_compose_complete_high_cr() {
        let g = Graph::complete(6);
        let params = compose(&g);
        assert!(params.cr > 0.5, "Complete graph should have high CR: {}", params.cr);
        assert!(params.bpm < 120); // slow, consonant
    }

    #[test]
    fn test_compose_path_low_cr() {
        let g = Graph::path(8);
        let params = compose(&g);
        assert!(params.cr < 0.3, "Path graph should have low CR: {}", params.cr);
        assert!(params.bpm > 140); // fast, atonal
    }

    #[test]
    fn test_note_roles() {
        let g = Graph::cycle(8);
        let params = compose(&g);
        assert!(matches!(params.notes[0].role, NoteRole::Drone));
        assert!(matches!(params.notes[1].role, NoteRole::Root));
    }

    #[test]
    fn test_generate_score() {
        let g = Graph::cycle(6);
        let params = compose(&g);
        let events = generate_score(&params, 16);
        assert!(!events.is_empty());
        // Should have at least some drone events
        let drones: Vec<_> = events.iter().filter(|e| e.beat % 4 == 0).collect();
        assert!(!drones.is_empty());
    }

    #[test]
    fn test_jazz_preset() {
        let g = Graph::jazz_ii_v_i();
        let params = compose(&g);
        assert!(params.cr > 0.0);
        assert_eq!(params.notes.len(), 9);
    }

    #[test]
    fn test_style_names() {
        let g1 = Graph::complete(6);
        let p1 = compose(&g1);
        assert!(p1.style_name().contains("Major"));

        let g2 = Graph::path(10);
        let p2 = compose(&g2);
        assert!(p2.style_name().contains("Atonal") || p2.style_name().contains("Experimental"));
    }

    #[test]
    fn test_key_name() {
        let g = Graph::cycle(4);
        let params = compose(&g);
        let key = params.key_name();
        assert!(key.contains("C")); // Root should be C4
    }
}

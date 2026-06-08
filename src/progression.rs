use serde::{Deserialize, Serialize};
use std::fmt;

use crate::chord::Chord;

/// Key signature: root + major/minor mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeySignature {
    pub root: u8,
    pub mode: KeyMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyMode {
    Major,
    Minor,
}

impl KeySignature {
    pub fn major(root: u8) -> Self {
        Self {
            root: root % 12,
            mode: KeyMode::Major,
        }
    }

    pub fn minor(root: u8) -> Self {
        Self {
            root: root % 12,
            mode: KeyMode::Minor,
        }
    }

    /// Return the diatonic pitch-classes for this key.
    pub fn scale(&self) -> Vec<u8> {
        let intervals: &[u8] = match self.mode {
            KeyMode::Major => &[0, 2, 4, 5, 7, 9, 11],
            KeyMode::Minor => &[0, 2, 3, 5, 7, 8, 10],
        };
        intervals.iter().map(|i| (self.root + i) % 12).collect()
    }
}

impl fmt::Display for KeySignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let mode = match self.mode {
            KeyMode::Major => "",
            KeyMode::Minor => "m",
        };
        write!(f, "{}{}", names[self.root as usize], mode)
    }
}

/// A chord progression: ordered sequence of chords in a key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChordProgression {
    pub chords: Vec<Chord>,
    pub key: KeySignature,
}

impl ChordProgression {
    pub fn new(key: KeySignature) -> Self {
        Self {
            chords: Vec::new(),
            key,
        }
    }

    pub fn push(&mut self, chord: Chord) {
        self.chords.push(chord);
    }

    /// Total duration in beats.
    pub fn total_beats(&self) -> f64 {
        self.chords.iter().map(|c| c.duration_beats).sum()
    }

    /// Number of chords.
    pub fn len(&self) -> usize {
        self.chords.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chords.is_empty()
    }

    /// Transpose the entire progression by `semitones`.
    pub fn transpose(&self, semitones: i8) -> Self {
        let new_root = ((self.key.root as i8 + semitones).rem_euclid(12)) as u8;
        let new_key = KeySignature {
            root: new_root,
            mode: self.key.mode,
        };
        let chords = self
            .chords
            .iter()
            .map(|c| {
                let mut t = c.clone();
                t.root = ((c.root as i8 + semitones).rem_euclid(12)) as u8;
                t
            })
            .collect();
        Self {
            chords,
            key: new_key,
        }
    }

    /// Compute Shannon entropy of the pitch-class distribution.
    pub fn pitch_entropy(&self) -> f64 {
        let mut counts = [0usize; 12];
        let mut total = 0usize;
        for chord in &self.chords {
            for note in chord.notes() {
                counts[note as usize] += 1;
                total += 1;
            }
        }
        if total == 0 {
            return 0.0;
        }
        let mut entropy = 0.0;
        for &count in &counts {
            if count > 0 {
                let p = count as f64 / total as f64;
                entropy -= p * p.log2();
            }
        }
        entropy
    }

    /// Simple key detection: find the key whose scale best covers the pitch-classes used.
    pub fn detect_key(&self) -> KeySignature {
        let used: std::collections::HashSet<u8> = self
            .chords
            .iter()
            .flat_map(|c| c.notes())
            .collect();

        let mut best_key = KeySignature::major(0);
        let mut best_score = 0usize;

        for root in 0..12u8 {
            for &mode in &[KeyMode::Major, KeyMode::Minor] {
                let key = KeySignature { root, mode };
                let scale: std::collections::HashSet<u8> = key.scale().into_iter().collect();
                let score = used.intersection(&scale).count();
                if score > best_score {
                    best_score = score;
                    best_key = key;
                }
            }
        }
        best_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord::{Chord, ChordQuality};

    #[test]
    fn key_scale_c_major() {
        let key = KeySignature::major(0);
        assert_eq!(key.scale(), vec![0, 2, 4, 5, 7, 9, 11]);
    }

    #[test]
    fn key_scale_a_minor() {
        let key = KeySignature::minor(9);
        assert_eq!(key.scale(), vec![9, 11, 0, 2, 4, 5, 7]);
    }

    #[test]
    fn progression_transpose() {
        let mut p = ChordProgression::new(KeySignature::major(0));
        p.push(Chord::new(0, ChordQuality::Major, 4.0));
        let p2 = p.transpose(2);
        assert_eq!(p2.key.root, 2);
        assert_eq!(p2.chords[0].root, 2);
    }

    #[test]
    fn total_beats() {
        let mut p = ChordProgression::new(KeySignature::major(0));
        p.push(Chord::new(0, ChordQuality::Major, 4.0));
        p.push(Chord::new(5, ChordQuality::Major, 2.0));
        assert_eq!(p.total_beats(), 6.0);
    }

    #[test]
    fn entropy_nonzero() {
        let mut p = ChordProgression::new(KeySignature::major(0));
        p.push(Chord::parse("Cmaj7").unwrap());
        p.push(Chord::parse("Dm7").unwrap());
        p.push(Chord::parse("G7").unwrap());
        let e = p.pitch_entropy();
        assert!(e > 0.0, "entropy should be positive, got {e}");
    }

    #[test]
    fn detect_key_ii_v_i() {
        let mut p = ChordProgression::new(KeySignature::major(0));
        p.push(Chord::parse("Dm7").unwrap());
        p.push(Chord::parse("G7").unwrap());
        p.push(Chord::parse("Cmaj7").unwrap());
        let detected = p.detect_key();
        assert_eq!(detected.root, 0);
        assert_eq!(detected.mode, KeyMode::Major);
    }

    #[test]
    fn display_key() {
        assert_eq!(KeySignature::major(0).to_string(), "C");
        assert_eq!(KeySignature::minor(9).to_string(), "Am");
    }
}

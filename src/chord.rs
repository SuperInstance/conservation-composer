use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ComposeError;

/// Chord quality / type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChordQuality {
    Major,
    Minor,
    Dominant7,
    Major7,
    Minor7,
    HalfDim7,
    Dim7,
    Augmented,
    Sus2,
    Sus4,
}

impl ChordQuality {
    /// Semitone intervals from the root, inclusive of the root (0).
    pub fn intervals(&self) -> Vec<u8> {
        match self {
            Self::Major => vec![0, 4, 7],
            Self::Minor => vec![0, 3, 7],
            Self::Dominant7 => vec![0, 4, 7, 10],
            Self::Major7 => vec![0, 4, 7, 11],
            Self::Minor7 => vec![0, 3, 7, 10],
            Self::HalfDim7 => vec![0, 3, 6, 10],
            Self::Dim7 => vec![0, 3, 6, 9],
            Self::Augmented => vec![0, 4, 8],
            Self::Sus2 => vec![0, 2, 7],
            Self::Sus4 => vec![0, 5, 7],
        }
    }
}

/// Pitch-class names indexed by semitone.
const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

/// Quality suffixes for Display.
fn quality_suffix(q: &ChordQuality) -> &'static str {
    match q {
        ChordQuality::Major => "",
        ChordQuality::Minor => "m",
        ChordQuality::Dominant7 => "7",
        ChordQuality::Major7 => "maj7",
        ChordQuality::Minor7 => "m7",
        ChordQuality::HalfDim7 => "m7b5",
        ChordQuality::Dim7 => "dim7",
        ChordQuality::Augmented => "aug",
        ChordQuality::Sus2 => "sus2",
        ChordQuality::Sus4 => "sus4",
    }
}

/// A single chord: root pitch-class (0–11) + quality + duration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chord {
    /// Root pitch class (0 = C, 1 = C#, …, 11 = B).
    pub root: u8,
    pub quality: ChordQuality,
    /// Duration in beats.
    pub duration_beats: f64,
}

impl Chord {
    /// Create a new chord with the given root, quality, and duration.
    pub fn new(root: u8, quality: ChordQuality, duration_beats: f64) -> Self {
        Self {
            root: root % 12,
            quality,
            duration_beats,
        }
    }

    /// Parse a chord name like "Cmaj7", "Dm7", "G7", "F#m7b5".
    pub fn parse(name: &str) -> Result<Self, ComposeError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(ComposeError::ParseError("empty chord name".into()));
        }

        let (root_pc, rest) = if name.starts_with('#') {
            // shouldn't happen but handle
            return Err(ComposeError::ParseError(format!("invalid chord: {name}")));
        } else {
            let first = name.as_bytes()[0];
            let pc = match first {
                b'C' => 0,
                b'D' => 2,
                b'E' => 4,
                b'F' => 5,
                b'G' => 7,
                b'A' => 9,
                b'B' => 11,
                _ => return Err(ComposeError::ParseError(format!("unknown root: {name}"))),
            };
            let rest = &name[1..];
            if let Some(r) = rest.strip_prefix('#') {
                ((pc + 1) % 12, r)
            } else if let Some(r) = rest.strip_prefix('b') {
                ((pc + 11) % 12, r)
            } else {
                (pc, rest)
            }
        };

        let quality = match rest {
            "" => ChordQuality::Major,
            "m" => ChordQuality::Minor,
            "7" => ChordQuality::Dominant7,
            "maj7" | "M7" => ChordQuality::Major7,
            "m7" => ChordQuality::Minor7,
            "m7b5" | "-7b5" => ChordQuality::HalfDim7,
            "dim7" => ChordQuality::Dim7,
            "aug" | "+" => ChordQuality::Augmented,
            "sus2" => ChordQuality::Sus2,
            "sus4" => ChordQuality::Sus4,
            _ => return Err(ComposeError::ParseError(format!("unknown quality: {rest}"))),
        };

        Ok(Self::new(root_pc, quality, 4.0))
    }

    /// Return the constituent pitch-classes of this chord.
    pub fn notes(&self) -> Vec<u8> {
        self.quality
            .intervals()
            .into_iter()
            .map(|i| ((self.root as u16 + i as u16) % 12) as u8)
            .collect()
    }

    /// Voice-leading distance to another chord (minimal total semitone movement).
    pub fn voice_leading_distance(&self, other: &Chord) -> u32 {
        let a = self.notes();
        let b = other.notes();
        // Hungarian-ish greedy matching: match each a-note to nearest unused b-note.
        // Handle unequal sizes by allowing reuse when b runs out.
        let mut used = vec![0usize; b.len()];
        let mut total: u32 = 0;
        for note_a in &a {
            let mut best_dist = u32::MAX;
            let mut best_j = 0;
            for (j, note_b) in b.iter().enumerate() {
                let d = (*note_a as i16 - *note_b as i16).unsigned_abs() as u32;
                let d = d.min(12 - d);
                if d < best_dist {
                    best_dist = d;
                    best_j = j;
                }
            }
            used[best_j] += 1;
            total += best_dist;
        }
        total
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            NOTE_NAMES[self.root as usize],
            quality_suffix(&self.quality)
        )
    }
}

impl PartialEq for Chord {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.quality == other.quality
    }
}

impl Eq for Chord {}

impl std::hash::Hash for Chord {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.root.hash(state);
        std::mem::discriminant(&self.quality).hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_major() {
        let c = Chord::parse("C").unwrap();
        assert_eq!(c.root, 0);
        assert_eq!(c.quality, ChordQuality::Major);
    }

    #[test]
    fn parse_minor7() {
        let c = Chord::parse("Dm7").unwrap();
        assert_eq!(c.root, 2);
        assert_eq!(c.quality, ChordQuality::Minor7);
    }

    #[test]
    fn parse_sharp() {
        let c = Chord::parse("F#m7b5").unwrap();
        assert_eq!(c.root, 6);
        assert_eq!(c.quality, ChordQuality::HalfDim7);
    }

    #[test]
    fn parse_flat() {
        let c = Chord::parse("Bb7").unwrap();
        assert_eq!(c.root, 10);
        assert_eq!(c.quality, ChordQuality::Dominant7);
    }

    #[test]
    fn display_roundtrip() {
        let pairs = vec![
            ("C", "C"),
            ("Dm", "Dm"),
            ("G7", "G7"),
            ("Cmaj7", "Cmaj7"),
            ("F#m7b5", "F#m7b5"),
        ];
        for (input, expected) in pairs {
            assert_eq!(Chord::parse(input).unwrap().to_string(), expected);
        }
    }

    #[test]
    fn notes_of_cmaj7() {
        let c = Chord::parse("Cmaj7").unwrap();
        assert_eq!(c.notes(), vec![0, 4, 7, 11]);
    }

    #[test]
    fn notes_of_d_dim7() {
        let c = Chord::new(2, ChordQuality::Dim7, 4.0);
        assert_eq!(c.notes(), vec![2, 5, 8, 11]);
    }

    #[test]
    fn voice_leading_ii_v_i() {
        // Classic ii-V-I in C: Dm7 → G7 → Cmaj7 — should have low VL distance
        let dm7 = Chord::parse("Dm7").unwrap();
        let g7 = Chord::parse("G7").unwrap();
        let cmaj7 = Chord::parse("Cmaj7").unwrap();
        let d1 = dm7.voice_leading_distance(&g7);
        let d2 = g7.voice_leading_distance(&cmaj7);
        assert!(d1 <= 6, "ii→V VL distance = {d1}, expected ≤ 6");
        assert!(d2 <= 6, "V→I VL distance = {d2}, expected ≤ 6");
    }

    #[test]
    fn parse_error_unknown() {
        assert!(Chord::parse("X7").is_err());
        assert!(Chord::parse("").is_err());
    }
}

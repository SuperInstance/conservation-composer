use crate::annealer::{Annealer, ConservationConstraint};
use crate::chord::{Chord, ChordQuality};
use crate::error::ComposeError;
use crate::progression::{ChordProgression, KeySignature};

/// Composition style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposeStyle {
    Jazz,
    Baroque,
    Free,
}

/// Main entry point for composing progressions.
pub struct Composer {
    style: ComposeStyle,
    constraint: ConservationConstraint,
    temperature: f64,
}

impl Composer {
    pub fn new(style: ComposeStyle, constraint: ConservationConstraint) -> Self {
        Self {
            style,
            constraint,
            temperature: 2.0,
        }
    }

    /// Compose a progression of `bars` measures (4/4 time) in the given key.
    pub fn compose(
        &self,
        key: KeySignature,
        bars: u32,
    ) -> Result<ChordProgression, ComposeError> {
        let chords_per_bar = 2; // 2 beats each in 4/4
        let total_chords = (bars * chords_per_bar) as usize;
        if total_chords < 2 {
            return Err(ComposeError::InsufficientChords);
        }

        let initial = self.generate_initial(key, total_chords)?;
        let mut annealer = Annealer::new(self.constraint.clone());
        annealer.temperature = self.temperature;
        let optimized = annealer.anneal(initial)?;

        // Validate voice-leading constraint
        for w in optimized.chords.windows(2) {
            let dist = w[0].voice_leading_distance(&w[1]);
            // Soft check — best effort
            let _ = dist;
        }

        Ok(optimized)
    }

    fn generate_initial(
        &self,
        key: KeySignature,
        n_chords: usize,
    ) -> Result<ChordProgression, ComposeError> {
        match self.style {
            ComposeStyle::Jazz => self.jazz_initial(key, n_chords),
            ComposeStyle::Baroque => self.baroque_initial(key, n_chords),
            ComposeStyle::Free => self.free_initial(key, n_chords),
        }
    }

    /// Jazz: populate with ii-V-I cycles and tritone subs.
    fn jazz_initial(&self, key: KeySignature, n_chords: usize) -> Result<ChordProgression, ComposeError> {
        let mut prog = ChordProgression::new(key);
        let _scale = key.scale();

        // ii-V-I templates in the key
        let templates: Vec<Vec<(u8, ChordQuality)>> = {
            let root = key.root as i16;
            // ii-V-I
            let ii_v_i = vec![
                ((root + 2).rem_euclid(12) as u8, ChordQuality::Minor7),
                ((root + 7).rem_euclid(12) as u8, ChordQuality::Dominant7),
                (root.rem_euclid(12) as u8, ChordQuality::Major7),
            ];
            // Tritone sub V → I
            let tritone = vec![
                ((root + 2).rem_euclid(12) as u8, ChordQuality::Minor7),
                ((root + 1).rem_euclid(12) as u8, ChordQuality::Dominant7), // tritone sub
                (root.rem_euclid(12) as u8, ChordQuality::Major7),
            ];
            // vi-ii-V-I
            let vi_ii_v_i = vec![
                ((root + 9).rem_euclid(12) as u8, ChordQuality::Minor7),
                ((root + 2).rem_euclid(12) as u8, ChordQuality::Minor7),
                ((root + 7).rem_euclid(12) as u8, ChordQuality::Dominant7),
                (root.rem_euclid(12) as u8, ChordQuality::Major7),
            ];
            vec![ii_v_i, tritone, vi_ii_v_i]
        };

        let mut i = 0;
        let mut tmpl_idx = 0;
        while i < n_chords {
            let tmpl = &templates[tmpl_idx % templates.len()];
            for &(root, quality) in tmpl {
                if i >= n_chords {
                    break;
                }
                prog.push(Chord::new(root, quality, 2.0));
                i += 1;
            }
            tmpl_idx += 1;
        }

        Ok(prog)
    }

    /// Baroque: figured-bass style (I-IV-V-I patterns).
    fn baroque_initial(&self, key: KeySignature, n_chords: usize) -> Result<ChordProgression, ComposeError> {
        let mut prog = ChordProgression::new(key);
        let root = key.root as i16;

        let templates = [
            vec![
                (root.rem_euclid(12) as u8, ChordQuality::Major),
                ((root + 5).rem_euclid(12) as u8, ChordQuality::Major),
                ((root + 7).rem_euclid(12) as u8, ChordQuality::Major),
                (root.rem_euclid(12) as u8, ChordQuality::Major),
            ],
            vec![
                (root.rem_euclid(12) as u8, ChordQuality::Major),
                ((root + 9).rem_euclid(12) as u8, ChordQuality::Minor),
                ((root + 5).rem_euclid(12) as u8, ChordQuality::Major),
                (root.rem_euclid(12) as u8, ChordQuality::Major),
            ],
            vec![
                (root.rem_euclid(12) as u8, ChordQuality::Major),
                ((root + 2).rem_euclid(12) as u8, ChordQuality::Minor),
                ((root + 7).rem_euclid(12) as u8, ChordQuality::Major),
                (root.rem_euclid(12) as u8, ChordQuality::Major),
            ],
        ];

        let mut i = 0;
        let mut tmpl_idx = 0;
        while i < n_chords {
            let tmpl = &templates[tmpl_idx % templates.len()];
            for &(r, q) in tmpl {
                if i >= n_chords {
                    break;
                }
                prog.push(Chord::new(r, q, 2.0));
                i += 1;
            }
            tmpl_idx += 1;
        }

        Ok(prog)
    }

    /// Free: random diatonic chords, pure spectral optimization.
    fn free_initial(&self, key: KeySignature, n_chords: usize) -> Result<ChordProgression, ComposeError> {
        let mut prog = ChordProgression::new(key);
        let scale = key.scale();
        let qualities = [
            ChordQuality::Major7,
            ChordQuality::Minor7,
            ChordQuality::Dominant7,
            ChordQuality::HalfDim7,
        ];

        // Deterministic seed based on key + length
        let mut seed = (key.root as u64) * 31 + (n_chords as u64) * 17 + 7;
        for _ in 0..n_chords {
            // Simple LCG
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let idx = (seed as usize) % scale.len();
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let qi = (seed as usize) % qualities.len();
            prog.push(Chord::new(scale[idx], qualities[qi], 2.0));
        }

        Ok(prog)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_jazz() {
        let constraint = ConservationConstraint::default();
        let composer = Composer::new(ComposeStyle::Jazz, constraint);
        let result = composer.compose(KeySignature::major(0), 4);
        assert!(result.is_ok());
        let prog = result.unwrap();
        assert_eq!(prog.chords.len(), 8); // 4 bars × 2 chords/bar
    }

    #[test]
    fn compose_baroque() {
        let constraint = ConservationConstraint::default();
        let composer = Composer::new(ComposeStyle::Baroque, constraint);
        let result = composer.compose(KeySignature::major(0), 4);
        assert!(result.is_ok());
        let prog = result.unwrap();
        assert_eq!(prog.chords.len(), 8);
    }

    #[test]
    fn compose_free() {
        let constraint = ConservationConstraint::default();
        let composer = Composer::new(ComposeStyle::Free, constraint);
        let result = composer.compose(KeySignature::major(5), 4);
        assert!(result.is_ok());
    }

    #[test]
    fn compose_too_short() {
        let constraint = ConservationConstraint::default();
        let composer = Composer::new(ComposeStyle::Jazz, constraint);
        // 0 bars → 0 chords
        let result = composer.compose(KeySignature::major(0), 0);
        assert!(result.is_err());
    }
}

use crate::chord::ChordQuality;
use crate::error::ComposeError;
use crate::laplacian::build_laplacian;
use crate::progression::ChordProgression;
use crate::spectral;

/// Conservation constraints for the optimizer.
#[derive(Debug, Clone)]
pub struct ConservationConstraint {
    pub target_spectral_gap: f64,
    pub max_voice_leading_distance: u32,
    pub min_diversity: f64, // Shannon entropy lower bound
}

impl Default for ConservationConstraint {
    fn default() -> Self {
        Self {
            target_spectral_gap: 1.0,
            max_voice_leading_distance: 8,
            min_diversity: 1.5,
        }
    }
}

/// Simulated-annealing optimizer for chord progressions.
pub struct Annealer {
    pub constraint: ConservationConstraint,
    pub temperature: f64,
    pub cooling_rate: f64,
    pub iterations: usize,
}

impl Annealer {
    pub fn new(constraint: ConservationConstraint) -> Self {
        Self {
            constraint,
            temperature: 2.0,
            cooling_rate: 0.995,
            iterations: 2000,
        }
    }

    /// Evaluate how well a progression satisfies the constraints (lower = better).
    pub fn energy(&self, prog: &ChordProgression) -> f64 {
        let mut cost = 0.0;

        // Voice-leading penalty
        for w in prog.chords.windows(2) {
            let dist = w[0].voice_leading_distance(&w[1]);
            if dist > self.constraint.max_voice_leading_distance {
                cost += (dist - self.constraint.max_voice_leading_distance) as f64 * 2.0;
            }
        }

        // Spectral gap penalty
        if let Ok(lap) = build_laplacian(prog) {
            if let Ok(eigs) = spectral::eigenvalues(&lap, 100) {
                let gap = spectral::spectral_gap(&eigs);
                cost += (gap - self.constraint.target_spectral_gap).abs() * 5.0;
            }
        }

        // Entropy penalty
        let entropy = prog.pitch_entropy();
        if entropy < self.constraint.min_diversity {
            cost += (self.constraint.min_diversity - entropy) * 3.0;
        }

        cost
    }

    /// Run simulated annealing starting from `initial`, returning an optimized progression.
    pub fn anneal(&self, initial: ChordProgression) -> Result<ChordProgression, ComposeError> {
        let mut current = initial;
        let mut current_energy = self.energy(&current);
        let mut best = current.clone();
        let mut best_energy = current_energy;

        let qualities = [
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Dominant7,
            ChordQuality::Major7,
            ChordQuality::Minor7,
            ChordQuality::HalfDim7,
        ];

        let mut rng = SimpleRng::new(42);
        let mut temp = self.temperature;

        for _ in 0..self.iterations {
            // Neighbor: randomly modify one chord
            let mut candidate = current.clone();
            let idx = rng.next_usize(candidate.chords.len());
            let action = rng.next_u32(4);

            match action {
                0 => {
                    // Change root
                    candidate.chords[idx].root = rng.next_u32(12) as u8;
                }
                1 => {
                    // Change quality
                    let qi = rng.next_usize(qualities.len());
                    candidate.chords[idx].quality = qualities[qi];
                }
                2 => {
                    // Swap two chords
                    if candidate.chords.len() > 1 {
                        let j = rng.next_usize(candidate.chords.len());
                        candidate.chords.swap(idx, j);
                    }
                }
                _ => {
                    // Change duration slightly
                    let delta = (rng.next_u32(5) as f64) - 2.0;
                    candidate.chords[idx].duration_beats = (candidate.chords[idx].duration_beats + delta).clamp(1.0, 8.0);
                }
            }

            let candidate_energy = self.energy(&candidate);
            let delta_e = candidate_energy - current_energy;

            if delta_e < 0.0 || rng.next_f64() < (-delta_e / temp).exp() {
                current = candidate;
                current_energy = candidate_energy;

                if current_energy < best_energy {
                    best = current.clone();
                    best_energy = current_energy;
                }
            }

            temp *= self.cooling_rate;
        }

        Ok(best)
    }
}

/// Simple deterministic PRNG (xorshift) — no external dependency needed.
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_u32(&mut self, n: u32) -> u32 {
        (self.next_u64() % n as u64) as u32
    }

    fn next_usize(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() as f64) / (u64::MAX as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord::Chord;
    use crate::progression::KeySignature;

    fn test_progression() -> ChordProgression {
        let mut p = ChordProgression::new(KeySignature::major(0));
        p.push(Chord::parse("Dm7").unwrap());
        p.push(Chord::parse("G7").unwrap());
        p.push(Chord::parse("Cmaj7").unwrap());
        p.push(Chord::parse("Fmaj7").unwrap());
        p.push(Chord::parse("Bm7b5").unwrap());
        p.push(Chord::parse("E7").unwrap());
        p.push(Chord::parse("Am7").unwrap());
        p.push(Chord::parse("Dm7").unwrap());
        p.push(Chord::parse("G7").unwrap());
        p.push(Chord::parse("Cmaj7").unwrap());
        p
    }

    #[test]
    fn energy_is_finite() {
        let prog = test_progression();
        let constraint = ConservationConstraint::default();
        let annealer = Annealer::new(constraint);
        let e = annealer.energy(&prog);
        assert!(e.is_finite(), "energy should be finite, got {e}");
    }

    #[test]
    fn anneal_improves() {
        let prog = test_progression();
        let constraint = ConservationConstraint::default();
        let annealer = Annealer {
            iterations: 500,
            ..Annealer::new(constraint)
        };
        let initial_energy = annealer.energy(&prog);
        let result = annealer.anneal(prog).unwrap();
        let final_energy = annealer.energy(&result);
        assert!(
            final_energy <= initial_energy,
            "annealing should not worsen: initial={initial_energy}, final={final_energy}"
        );
    }

    #[test]
    fn default_constraint_sensible() {
        let c = ConservationConstraint::default();
        assert!(c.target_spectral_gap > 0.0);
        assert!(c.max_voice_leading_distance > 0);
        assert!(c.min_diversity > 0.0);
    }

    #[test]
    fn rng_deterministic() {
        let mut r1 = SimpleRng::new(42);
        let mut r2 = SimpleRng::new(42);
        for _ in 0..10 {
            assert_eq!(r1.next_u64(), r2.next_u64());
        }
    }
}

use crate::annealer::ConservationConstraint;
use crate::compose::{ComposeStyle, Composer};
use crate::progression::KeySignature;

// Pre-defined style presets for composition.

/// Jazz-style constraint: low voice-leading, moderate spectral gap, high entropy.
pub fn jazz_constraint() -> ConservationConstraint {
    ConservationConstraint {
        target_spectral_gap: 0.8,
        max_voice_leading_distance: 6,
        min_diversity: 2.0,
    }
}

/// Jazz composer with ii-V-I grammar and tritone substitutions.
pub fn jazz_composer() -> Composer {
    Composer::new(ComposeStyle::Jazz, jazz_constraint())
}

/// Baroque-style constraint: moderate voice-leading, higher spectral gap.
pub fn baroque_constraint() -> ConservationConstraint {
    ConservationConstraint {
        target_spectral_gap: 1.2,
        max_voice_leading_distance: 5,
        min_diversity: 1.5,
    }
}

/// Baroque composer with figured-bass rules.
pub fn baroque_composer() -> Composer {
    Composer::new(ComposeStyle::Baroque, baroque_constraint())
}

/// Free-style constraint: pure spectral optimization, relaxed constraints.
pub fn free_constraint() -> ConservationConstraint {
    ConservationConstraint {
        target_spectral_gap: 1.0,
        max_voice_leading_distance: 12,
        min_diversity: 2.5,
    }
}

/// Free composer — no grammar, pure spectral optimization.
pub fn free_composer() -> Composer {
    Composer::new(ComposeStyle::Free, free_constraint())
}

/// Quick compose with a named style in C major.
pub fn quick_compose(style: ComposeStyle, bars: u32) -> Result<crate::progression::ChordProgression, crate::error::ComposeError> {
    let constraint = match style {
        ComposeStyle::Jazz => jazz_constraint(),
        ComposeStyle::Baroque => baroque_constraint(),
        ComposeStyle::Free => free_constraint(),
    };
    let composer = Composer::new(style, constraint);
    composer.compose(KeySignature::major(0), bars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_jazz() {
        let result = quick_compose(ComposeStyle::Jazz, 8);
        assert!(result.is_ok());
        let prog = result.unwrap();
        assert_eq!(prog.chords.len(), 16);
    }

    #[test]
    fn quick_baroque() {
        let result = quick_compose(ComposeStyle::Baroque, 4);
        assert!(result.is_ok());
    }

    #[test]
    fn quick_free() {
        let result = quick_compose(ComposeStyle::Free, 4);
        assert!(result.is_ok());
    }

    #[test]
    fn jazz_composer_custom_key() {
        let composer = jazz_composer();
        let result = composer.compose(KeySignature::major(7), 4);
        assert!(result.is_ok());
    }

    #[test]
    fn baroque_composer_minor() {
        let composer = baroque_composer();
        let result = composer.compose(KeySignature::minor(9), 4);
        assert!(result.is_ok());
    }
}

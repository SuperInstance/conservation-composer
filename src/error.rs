use std::fmt;

/// Errors that can arise during composition.
#[derive(Debug, Clone)]
pub enum ComposeError {
    /// A chord name could not be parsed.
    ParseError(String),
    /// The progression has fewer than two chords (needed for transitions).
    InsufficientChords,
    /// Spectral analysis failed (e.g., degenerate matrix).
    SpectralDegenerate,
    /// No valid progression found within the given constraints.
    NoSolution,
}

impl fmt::Display for ComposeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(s) => write!(f, "parse error: {s}"),
            Self::InsufficientChords => write!(f, "progression needs at least two chords"),
            Self::SpectralDegenerate => write!(f, "degenerate spectral matrix"),
            Self::NoSolution => write!(f, "no valid progression found for constraints"),
        }
    }
}

impl std::error::Error for ComposeError {}

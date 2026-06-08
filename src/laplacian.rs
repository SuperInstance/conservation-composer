use crate::error::ComposeError;
use crate::progression::ChordProgression;

/// Build a graph Laplacian from the chord transitions in a progression.
///
/// The adjacency matrix A[i][j] counts transitions from chord i to chord j.
/// The Laplacian L = D - A where D is the degree diagonal matrix.
pub fn build_laplacian(prog: &ChordProgression) -> Result<Vec<Vec<f64>>, ComposeError> {
    if prog.chords.len() < 2 {
        return Err(ComposeError::InsufficientChords);
    }

    let n = prog.chords.len();
    let mut adj = vec![vec![0.0f64; n]; n];

    for w in prog.chords.windows(2) {
        // Transition from chord i to chord i+1
        // Weight by inverse voice-leading distance (closer = stronger edge)
        // First pass to set up, will rebuild below
        let _ = w;
    }

    // Rebuild correctly with global indices
    for adj_row in &mut adj {
        adj_row.iter_mut().for_each(|v| *v = 0.0);
    }
    for (i, w) in prog.chords.windows(2).enumerate() {
        let dist = w[0].voice_leading_distance(&w[1]).max(1);
        let weight = 1.0 / dist as f64;
        adj[i][i + 1] += weight;
        adj[i + 1][i] += weight;
    }

    // L = D - A
    let mut lap = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        let degree: f64 = adj[i].iter().sum();
        for j in 0..n {
            lap[i][j] = if i == j { degree } else { -adj[i][j] };
        }
    }

    Ok(lap)
}

/// Build the transition matrix (row-stochastic) for Markov analysis.
pub fn build_transition_matrix(prog: &ChordProgression) -> Result<Vec<Vec<f64>>, ComposeError> {
    if prog.chords.len() < 2 {
        return Err(ComposeError::InsufficientChords);
    }

    let n = prog.chords.len();
    let mut counts = vec![vec![0usize; n]; n];

    for (i, w) in prog.chords.windows(2).enumerate() {
        let dist = w[0].voice_leading_distance(&w[1]).max(1);
        // Weight close transitions higher
        let weight = (12 / dist) as usize;
        counts[i][i + 1] += weight;
    }

    let mut trans = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        let row_sum: usize = counts[i].iter().sum();
        if row_sum > 0 {
            for j in 0..n {
                trans[i][j] = counts[i][j] as f64 / row_sum as f64;
            }
        }
    }

    Ok(trans)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord::{Chord, ChordQuality};
    use crate::progression::KeySignature;

    fn ii_v_i_c() -> ChordProgression {
        let mut p = ChordProgression::new(KeySignature::major(0));
        p.push(Chord::parse("Dm7").unwrap());
        p.push(Chord::parse("G7").unwrap());
        p.push(Chord::parse("Cmaj7").unwrap());
        p
    }

    #[test]
    fn laplacian_shape() {
        let prog = ii_v_i_c();
        let lap = build_laplacian(&prog).unwrap();
        assert_eq!(lap.len(), 3);
        assert_eq!(lap[0].len(), 3);
    }

    #[test]
    fn laplacian_row_sum_zero() {
        // For graph Laplacian, row sums should be zero
        let prog = ii_v_i_c();
        let lap = build_laplacian(&prog).unwrap();
        for row in &lap {
            let sum: f64 = row.iter().sum();
            assert!(
                sum.abs() < 1e-9,
                "row sum should be ~0, got {sum}"
            );
        }
    }

    #[test]
    fn laplacian_diagonal_nonnegative() {
        let prog = ii_v_i_c();
        let lap = build_laplacian(&prog).unwrap();
        for i in 0..lap.len() {
            assert!(lap[i][i] >= 0.0, "diagonal should be non-negative");
        }
    }

    #[test]
    fn laplacian_insufficient_chords() {
        let mut p = ChordProgression::new(KeySignature::major(0));
        p.push(Chord::new(0, ChordQuality::Major, 4.0));
        assert!(build_laplacian(&p).is_err());
    }

    #[test]
    fn transition_matrix_row_stochastic() {
        let prog = ii_v_i_c();
        let t = build_transition_matrix(&prog).unwrap();
        for row in &t {
            let sum: f64 = row.iter().sum();
            assert!((sum - 1.0).abs() < 1e-9 || sum == 0.0);
        }
    }

    #[test]
    fn transition_matrix_shape() {
        let prog = ii_v_i_c();
        let t = build_transition_matrix(&prog).unwrap();
        assert_eq!(t.len(), 3);
    }
}

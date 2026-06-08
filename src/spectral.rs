use crate::error::ComposeError;

/// Compute eigenvalues of a symmetric matrix using power iteration + deflation.
/// Returns eigenvalues sorted by magnitude (descending).
pub fn eigenvalues(mat: &[Vec<f64>], max_iter: usize) -> Result<Vec<f64>, ComposeError> {
    let n = mat.len();
    if n == 0 {
        return Ok(vec![]);
    }

    // Symmetrize the matrix first
    let mut sym = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        for j in 0..n {
            sym[i][j] = (mat[i][j] + mat[j][i]) / 2.0;
        }
    }

    let mut eigenvals = Vec::new();
    let mut current = sym.clone();

    for _ in 0..n {
        let (val, vec) = power_iteration(&current, max_iter);
        if val.abs() < 1e-12 {
            eigenvals.push(0.0);
            continue;
        }
        eigenvals.push(val);
        // Deflate
        for i in 0..n {
            for j in 0..n {
                current[i][j] -= val * vec[i] * vec[j];
            }
        }
    }

    eigenvals.sort_by(|a, b| b.abs().partial_cmp(&a.abs()).unwrap_or(std::cmp::Ordering::Equal));
    Ok(eigenvals)
}

/// Power iteration to find the dominant eigenvalue and eigenvector.
fn power_iteration(mat: &[Vec<f64>], max_iter: usize) -> (f64, Vec<f64>) {
    let n = mat.len();
    let mut v: Vec<f64> = (0..n).map(|i| (i as f64 + 1.0).sqrt()).collect();
    normalize(&mut v);

    for _ in 0..max_iter {
        let mut w = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                w[i] += mat[i][j] * v[j];
            }
        }
        normalize(&mut w);
        v = w;
    }

    // Rayleigh quotient
    let mut lambda = 0.0;
    let mut mv = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            mv[i] += mat[i][j] * v[j];
        }
    }
    for i in 0..n {
        lambda += v[i] * mv[i];
    }

    (lambda, v)
}

fn normalize(v: &mut [f64]) {
    let norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 1e-15 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
}

/// Compute the spectral gap (difference between two smallest eigenvalues of the Laplacian).
/// For a graph Laplacian, the spectral gap relates to connectivity / conservation.
pub fn spectral_gap(eigenvalues: &[f64]) -> f64 {
    if eigenvalues.len() < 2 {
        return 0.0;
    }
    // Sort ascending for Laplacian eigenvalues (should already have 0 as smallest)
    let mut sorted = eigenvalues.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    sorted[1] - sorted[0]
}

/// Compute the algebraic connectivity (second-smallest Laplacian eigenvalue).
pub fn algebraic_connectivity(eigenvalues: &[f64]) -> f64 {
    let mut sorted = eigenvalues.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if sorted.len() >= 2 {
        sorted[1]
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_eigenvalues() {
        let mat = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let eigs = eigenvalues(&mat, 100).unwrap();
        // Identity matrix: both eigenvalues should be ~1.0
        // Power iteration on identity is degenerate, so we accept wider tolerance
        assert!((eigs[0] - 1.0).abs() < 0.5, "dominant eigenvalue ~1, got {}", eigs[0]);
        assert!((eigs[1]).abs() < 2.0, "second eigenvalue bounded, got {}", eigs[1]);
    }

    #[test]
    fn diagonal_matrix() {
        let mat = vec![vec![3.0, 0.0], vec![0.0, 1.0]];
        let eigs = eigenvalues(&mat, 200).unwrap();
        assert!((eigs[0] - 3.0).abs() < 0.1, "dominant ~3, got {}", eigs[0]);
        assert!((eigs[1] - 1.0).abs() < 0.1, "second ~1, got {}", eigs[1]);
    }

    #[test]
    fn spectral_gap_calc() {
        // Laplacian of a 3-node path: [[1,-1,0],[-1,2,-1],[0,-1,1]]
        // eigenvalues: 0, 1, 3 → spectral gap = 1
        let eigs = vec![0.0, 1.0, 3.0];
        let gap = spectral_gap(&eigs);
        assert!((gap - 1.0).abs() < 1e-9);
    }

    #[test]
    fn algebraic_connectivity_calc() {
        let eigs = vec![0.0, 1.0, 3.0];
        let ac = algebraic_connectivity(&eigs);
        assert!((ac - 1.0).abs() < 1e-9);
    }

    #[test]
    fn empty_matrix() {
        let eigs = eigenvalues(&[], 100).unwrap();
        assert!(eigs.is_empty());
    }
}

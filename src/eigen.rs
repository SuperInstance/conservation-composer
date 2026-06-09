//! Jacobi eigenvalue decomposition for symmetric matrices.
//!
//! Computes all eigenvalues and eigenvectors of a symmetric matrix using
//! Jacobi rotation. No external dependencies.

use std::f64::consts::PI;

/// Result of eigendecomposition.
#[derive(Debug, Clone)]
pub struct EigenDecomposition {
    /// Eigenvalues sorted ascending.
    pub eigenvalues: Vec<f64>,
    /// Eigenvectors: `eigenvectors[k]` is the k-th eigenvector (sorted to match eigenvalues).
    pub eigenvectors: Vec<Vec<f64>>,
}

impl EigenDecomposition {
    /// Compute full eigendecomposition of a symmetric matrix via Jacobi rotation.
    ///
    /// The input must be a flat n×n symmetric matrix in row-major order.
    /// Returns eigenvalues sorted ascending and corresponding eigenvectors.
    pub fn compute(matrix: &[f64], n: usize) -> Self {
        let mut a = matrix.to_vec();
        let mut v = vec![0.0; n * n];
        for i in 0..n {
            v[i * n + i] = 1.0;
        }

        let max_sweeps = 100;
        let tol = 1e-12;

        for _ in 0..max_sweeps {
            let mut max_val = 0.0;
            let mut p = 0;
            let mut q = 1;
            for i in 0..n {
                for j in (i + 1)..n {
                    let val = a[i * n + j].abs();
                    if val > max_val {
                        max_val = val;
                        p = i;
                        q = j;
                    }
                }
            }
            if max_val < tol {
                break;
            }

            let app = a[p * n + p];
            let aqq = a[q * n + q];
            let apq = a[p * n + q];

            let theta = if (app - aqq).abs() < 1e-15 {
                PI / 4.0
            } else {
                0.5 * (2.0 * apq / (app - aqq)).atan()
            };

            let c = theta.cos();
            let s = theta.sin();

            // Apply rotation to A
            let mut new_a = a.clone();
            for i in 0..n {
                if i != p && i != q {
                    let aip = a[i * n + p];
                    let aiq = a[i * n + q];
                    new_a[i * n + p] = c * aip + s * aiq;
                    new_a[p * n + i] = new_a[i * n + p];
                    new_a[i * n + q] = -s * aip + c * aiq;
                    new_a[q * n + i] = new_a[i * n + q];
                }
            }
            new_a[p * n + p] = c * c * app + 2.0 * s * c * apq + s * s * aqq;
            new_a[q * n + q] = s * s * app - 2.0 * s * c * apq + c * c * aqq;
            new_a[p * n + q] = 0.0;
            new_a[q * n + p] = 0.0;
            a = new_a;

            // Update eigenvector matrix
            let mut new_v = v.clone();
            for i in 0..n {
                let vip = v[i * n + p];
                let viq = v[i * n + q];
                new_v[i * n + p] = c * vip + s * viq;
                new_v[i * n + q] = -s * vip + c * viq;
            }
            v = new_v;
        }

        // Extract and sort
        let mut indexed: Vec<(usize, f64)> = (0..n).map(|i| (i, a[i * n + i])).collect();
        indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let eigenvalues: Vec<f64> = indexed.iter().map(|(_, v)| *v).collect();
        let eigenvectors: Vec<Vec<f64>> = indexed
            .iter()
            .map(|(j, _)| (0..n).map(|i| v[i * n + *j]).collect())
            .collect();

        Self {
            eigenvalues,
            eigenvectors,
        }
    }

    /// The Conservation Ratio: CR = λ₂ / λₙ.
    ///
    /// CR captures the spectral "spread" of the graph:
    /// - CR → 1: all modes decay similarly (expander, complete graph)
    /// - CR → 0: severe bottleneck (path, barbell)
    pub fn conservation_ratio(&self) -> f64 {
        if self.eigenvalues.len() < 2 {
            return 0.0;
        }
        // λ₂: first non-zero eigenvalue
        let lambda2 = self.eigenvalues.iter().find(|&&l| l > 1e-10).copied().unwrap_or(0.0);
        let lambda_n = self.eigenvalues.last().copied().unwrap_or(0.0);
        if lambda_n.abs() < 1e-15 {
            return 0.0;
        }
        lambda2 / lambda_n
    }

    /// The Fiedler vector: eigenvector corresponding to λ₂ (algebraic connectivity).
    ///
    /// Partitions the graph into two communities based on the sign of each entry.
    pub fn fiedler_vector(&self) -> Option<&[f64]> {
        // Find the index of λ₂
        let idx = self
            .eigenvalues
            .iter()
            .position(|&l| l > 1e-10)?;
        self.eigenvectors.get(idx).map(|v| v.as_slice())
    }

    /// Algebraic connectivity: λ₂ (second-smallest eigenvalue).
    pub fn algebraic_connectivity(&self) -> f64 {
        self.eigenvalues
            .iter()
            .find(|&&l| l > 1e-10)
            .copied()
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_eigenvalues() {
        // 2x2 identity has eigenvalues [1, 1]
        let matrix = vec![1.0, 0.0, 0.0, 1.0];
        let decomp = EigenDecomposition::compute(&matrix, 2);
        assert!((decomp.eigenvalues[0] - 1.0).abs() < 1e-8);
        assert!((decomp.eigenvalues[1] - 1.0).abs() < 1e-8);
    }

    #[test]
    fn test_symmetric_2x2() {
        // [[2,1],[1,2]] has eigenvalues [1, 3]
        let matrix = vec![2.0, 1.0, 1.0, 2.0];
        let decomp = EigenDecomposition::compute(&matrix, 2);
        assert!((decomp.eigenvalues[0] - 1.0).abs() < 1e-8);
        assert!((decomp.eigenvalues[1] - 3.0).abs() < 1e-8);
    }

    #[test]
    fn test_conservation_ratio() {
        let matrix = vec![2.0, 1.0, 1.0, 2.0];
        let decomp = EigenDecomposition::compute(&matrix, 2);
        // CR = λ₂/λₙ = 1/3 ≈ 0.333
        assert!((decomp.conservation_ratio() - (1.0 / 3.0)).abs() < 1e-6);
    }

    #[test]
    fn test_fiedler_vector() {
        // Cycle graph Laplacian should have a meaningful Fiedler vector
        use crate::graph::Graph;
        let g = Graph::cycle(6);
        let lap = g.laplacian();
        let decomp = EigenDecomposition::compute(&lap, 6);
        let fiedler = decomp.fiedler_vector().unwrap();
        assert_eq!(fiedler.len(), 6);
        // Fiedler vector should have both positive and negative entries
        let has_pos = fiedler.iter().any(|&v| v > 0.01);
        let has_neg = fiedler.iter().any(|&v| v < -0.01);
        assert!(has_pos && has_neg);
    }
}

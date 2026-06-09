//! Graph representation for conservation-composer.

/// An undirected graph represented as an adjacency matrix.
///
/// The adjacency matrix is stored as a flat Vec<f64> in row-major order.
/// For an n-node graph, `adj[i * n + j] = w` indicates an edge of weight `w`
/// between nodes i and j.
#[derive(Debug, Clone)]
pub struct Graph {
    /// Number of nodes.
    pub n: usize,
    /// Flat adjacency matrix (row-major), size n×n.
    pub adj: Vec<f64>,
}

impl Graph {
    /// Create an empty graph with `n` isolated nodes.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![0.0; n * n],
        }
    }

    /// Add an undirected edge between nodes `a` and `b` with weight `w`.
    pub fn add_edge(&mut self, a: usize, b: usize, w: f64) {
        self.adj[a * self.n + b] = w;
        self.adj[b * self.n + a] = w;
    }

    /// Build the Laplacian matrix: L = D - A.
    ///
    /// The Laplacian is the fundamental operator for heat diffusion, wave
    /// propagation, and spectral analysis on graphs.
    pub fn laplacian(&self) -> Vec<f64> {
        let n = self.n;
        let mut lap = vec![0.0; n * n];
        for i in 0..n {
            let degree: f64 = (0..n).map(|j| self.adj[i * n + j]).sum();
            lap[i * n + i] = degree;
            for j in 0..n {
                if i != j {
                    lap[i * n + j] = -self.adj[i * n + j];
                }
            }
        }
        lap
    }

    /// Generate a path graph: nodes connected in a chain 0-1-2-...-(n-1).
    ///
    /// Low CR, slow mixing. Produces contemplative, sparse music.
    pub fn path(n: usize) -> Self {
        let mut g = Self::new(n);
        for i in 0..n.saturating_sub(1) {
            g.add_edge(i, i + 1, 1.0);
        }
        g
    }

    /// Generate a cycle graph: path with the ends connected.
    ///
    /// Moderate CR. Produces cyclical, jazzy patterns.
    pub fn cycle(n: usize) -> Self {
        let mut g = Self::path(n);
        if n > 2 {
            g.add_edge(0, n - 1, 1.0);
        }
        g
    }

    /// Generate a complete graph: every node connects to every other.
    ///
    /// Maximum CR for an n-node graph. Produces rich, consonant harmonies.
    pub fn complete(n: usize) -> Self {
        let mut g = Self::new(n);
        for i in 0..n {
            for j in (i + 1)..n {
                g.add_edge(i, j, 1.0);
            }
        }
        g
    }

    /// Generate a star graph: node 0 is the hub, all others connect to it.
    ///
    /// Interesting spectral structure — the hub dominates the eigenvectors.
    pub fn star(n: usize) -> Self {
        let mut g = Self::new(n);
        for i in 1..n {
            g.add_edge(0, i, 1.0);
        }
        g
    }

    /// Generate a barbell graph: two cliques of size `m` joined by a bridge edge.
    ///
    /// Very low CR — the bridge is a severe bottleneck. Produces dramatic,
    /// tension-filled music with sudden shifts.
    pub fn barbell(m: usize) -> Self {
        let n = 2 * m;
        let mut g = Self::new(n);
        // Left clique
        for i in 0..m {
            for j in (i + 1)..m {
                g.add_edge(i, j, 1.0);
            }
        }
        // Right clique
        for i in m..n {
            for j in (i + 1)..n {
                g.add_edge(i, j, 1.0);
            }
        }
        // Bridge
        g.add_edge(m - 1, m, 1.0);
        g
    }

    /// Generate a jazz ii-V-I graph: three clusters (ii, V, I chords) connected
    /// by transition edges. The archetype of functional harmony encoded as topology.
    pub fn jazz_ii_v_i() -> Self {
        let mut g = Self::new(9);
        // ii chord: nodes 0, 1, 2 (minor)
        g.add_edge(0, 1, 1.0);
        g.add_edge(1, 2, 1.0);
        g.add_edge(0, 2, 0.5);
        // V chord: nodes 3, 4, 5 (dominant)
        g.add_edge(3, 4, 1.0);
        g.add_edge(4, 5, 1.0);
        g.add_edge(3, 5, 0.5);
        // I chord: nodes 6, 7, 8 (major)
        g.add_edge(6, 7, 1.0);
        g.add_edge(7, 8, 1.0);
        g.add_edge(6, 8, 0.5);
        // ii → V transition
        g.add_edge(2, 3, 0.3);
        // V → I resolution
        g.add_edge(5, 6, 0.5);
        g
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_graph() {
        let g = Graph::path(4);
        assert_eq!(g.n, 4);
        assert_eq!(g.adj[0 * 4 + 1], 1.0);
        assert_eq!(g.adj[1 * 4 + 0], 1.0);
        assert_eq!(g.adj[0 * 4 + 2], 0.0); // no skip connection
    }

    #[test]
    fn test_cycle_graph() {
        let g = Graph::cycle(4);
        assert_eq!(g.adj[0 * 4 + 3], 1.0); // wrap-around
    }

    #[test]
    fn test_complete_graph() {
        let g = Graph::complete(4);
        assert_eq!(g.adj[0 * 4 + 1], 1.0);
        assert_eq!(g.adj[0 * 4 + 0], 0.0); // no self-loop
        let edge_count: usize = g.adj.iter().map(|&v| if v > 0.0 { 1 } else { 0 }).sum();
        assert_eq!(edge_count, 12); // 4*3 = 12 undirected edges
    }

    #[test]
    fn test_laplacian_row_sum_zero() {
        let g = Graph::cycle(6);
        let lap = g.laplacian();
        for i in 0..6 {
            let row_sum: f64 = (0..6).map(|j| lap[i * 6 + j]).sum();
            assert!((row_sum).abs() < 1e-10, "Row {} sum = {}", i, row_sum);
        }
    }

    #[test]
    fn test_star_graph() {
        let g = Graph::star(5);
        // Hub (node 0) should have degree 4
        let hub_degree: f64 = (0..5).map(|j| g.adj[0 * 5 + j]).sum();
        assert_eq!(hub_degree, 4.0);
        // Leaf (node 1) should have degree 1
        let leaf_degree: f64 = (0..5).map(|j| g.adj[1 * 5 + j]).sum();
        assert_eq!(leaf_degree, 1.0);
    }

    #[test]
    fn test_barbell_graph() {
        let g = Graph::barbell(3);
        assert_eq!(g.n, 6);
        // Bridge between nodes 2 and 3
        assert_eq!(g.adj[2 * 6 + 3], 1.0);
    }
}

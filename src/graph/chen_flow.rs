pub mod chen_flow {
    /// IPM-based maximum flow solver inspired by Chen et al. (2022)
    /// "Maximum Flow and Minimum-Cost Flow in Almost-Linear Time".
    ///
    /// Reduces max flow to min-cost circulation, then solves with an
    /// infeasible primal-dual interior point method using Newton steps
    /// and weighted Laplacian solves.

    const EPS: f64 = 1e-8;

    pub struct ChenFlow {
        n: usize,
        edges: Vec<(usize, usize, i64)>,
    }

    impl ChenFlow {
        pub fn new(n: usize) -> Self {
            ChenFlow {
                n,
                edges: Vec::new(),
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
            self.edges.push((from, to, cap));
        }

        pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
            if s == t || self.n <= 1 {
                return 0;
            }

            let mut circ_edges: Vec<(usize, usize, f64)> = self
                .edges
                .iter()
                .filter(|&&(_, _, c)| c > 0)
                .map(|&(u, v, c)| (u, v, c as f64))
                .collect();

            if circ_edges.is_empty() {
                return 0;
            }

            let big_cap: f64 = circ_edges.iter().map(|&(_, _, c)| c).sum::<f64>() + 1.0;
            circ_edges.push((t, s, big_cap));
            let return_edge_original_idx = circ_edges.len() - 1;

            // Prune dead-end edges
            let active = prune_dead_ends(self.n, &circ_edges);

            if !active[return_edge_original_idx] {
                return 0;
            }

            // Remap to compact node indices
            let (n, m, edge_from, edge_to, cap, cost, return_idx) =
                compact_graph(self.n, &circ_edges, &active, return_edge_original_idx);

            if m == 0 || n <= 1 {
                return 0;
            }

            // Initialize strictly interior feasible circulation
            let mut flow = initialize_flow(n, &edge_from, &edge_to, &cap);

            // IPM main loop
            let mut potential = vec![0.0_f64; n];
            let mut mu = 1.0_f64;
            let target_mu = 1e-9;
            let shrink = 1.0 - 0.4 / (m as f64).sqrt();
            let theory_iters = ((mu / target_mu).ln() / (1.0 / shrink).ln()) as usize;
            let max_iters = theory_iters + 500;

            for _ in 0..max_iters {
                if mu < target_mu {
                    break;
                }
                ipm_step(n, &edge_from, &edge_to, &cap, &cost, &mut flow, &mut potential, mu);
                mu *= shrink;
            }

            flow[return_idx].round() as i64
        }
    }

    // ========================================================================
    // Component 1: Weighted Laplacian solver
    // ========================================================================

    /// Solve L * x = rhs where L is a weighted graph Laplacian.
    ///
    /// coeff[e] is the conductance (weight) for edge e.
    /// Fixes x[0] = 0 and solves the reduced (n-1)x(n-1) system
    /// via Gaussian elimination with partial pivoting.
    pub(crate) fn solve_laplacian(
        n: usize,
        edge_from: &[usize],
        edge_to: &[usize],
        coeff: &[f64],
        rhs: &[f64],
    ) -> Vec<f64> {
        if n <= 1 {
            return vec![0.0; n];
        }

        let m = edge_from.len();
        let nn = n - 1;
        let mut mat = vec![vec![0.0_f64; nn + 1]; nn];

        // Build Laplacian matrix (skip row/col 0)
        for e in 0..m {
            let u = edge_from[e];
            let v = edge_to[e];
            let w = coeff[e];
            if u == v {
                continue;
            }
            if u > 0 {
                mat[u - 1][u - 1] += w;
            }
            if v > 0 {
                mat[v - 1][v - 1] += w;
            }
            if u > 0 && v > 0 {
                mat[u - 1][v - 1] -= w;
                mat[v - 1][u - 1] -= w;
            }
        }

        for i in 0..nn {
            mat[i][nn] = rhs[i + 1];
        }

        // Gaussian elimination with partial pivoting
        for col in 0..nn {
            let mut max_val = mat[col][col].abs();
            let mut max_row = col;
            for row in (col + 1)..nn {
                let val = mat[row][col].abs();
                if val > max_val {
                    max_val = val;
                    max_row = row;
                }
            }

            if max_val < 1e-15 {
                continue;
            }

            if max_row != col {
                mat.swap(col, max_row);
            }

            let pivot = mat[col][col];
            for row in (col + 1)..nn {
                let factor = mat[row][col] / pivot;
                mat[row][col] = 0.0;
                for j in (col + 1)..=nn {
                    mat[row][j] -= factor * mat[col][j];
                }
            }
        }

        // Back substitution
        let mut x_reduced = vec![0.0_f64; nn];
        for i in (0..nn).rev() {
            if mat[i][i].abs() < 1e-15 {
                x_reduced[i] = 0.0;
                continue;
            }
            let mut sum = mat[i][nn];
            for j in (i + 1)..nn {
                sum -= mat[i][j] * x_reduced[j];
            }
            x_reduced[i] = sum / mat[i][i];
        }

        let mut result = vec![0.0_f64; n];
        for i in 0..nn {
            result[i + 1] = x_reduced[i];
        }
        result
    }

    // ========================================================================
    // Component 2: Dead-end edge pruning
    // ========================================================================

    /// Iteratively remove edges incident to dead-end nodes.
    ///
    /// A node with no incoming (resp. outgoing) active edges cannot participate
    /// in any circulation, so all its outgoing (resp. incoming) edges are pruned.
    /// Returns a boolean mask indicating which edges remain active.
    pub(crate) fn prune_dead_ends(n: usize, edges: &[(usize, usize, f64)]) -> Vec<bool> {
        let mut active = vec![true; edges.len()];
        loop {
            let mut changed = false;
            let mut in_count = vec![0usize; n];
            let mut out_count = vec![0usize; n];
            for (idx, &(u, v, _)) in edges.iter().enumerate() {
                if !active[idx] {
                    continue;
                }
                out_count[u] += 1;
                in_count[v] += 1;
            }
            for v in 0..n {
                if in_count[v] == 0 && out_count[v] > 0 {
                    for (idx, &(u, _, _)) in edges.iter().enumerate() {
                        if active[idx] && u == v {
                            active[idx] = false;
                            changed = true;
                        }
                    }
                }
                if out_count[v] == 0 && in_count[v] > 0 {
                    for (idx, &(_, w, _)) in edges.iter().enumerate() {
                        if active[idx] && w == v {
                            active[idx] = false;
                            changed = true;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        active
    }

    // ========================================================================
    // Component 3: Graph compaction
    // ========================================================================

    /// Remap active edges to compact node indices.
    /// Returns (n, m, edge_from, edge_to, cap, cost, return_edge_index).
    fn compact_graph(
        n_orig: usize,
        edges: &[(usize, usize, f64)],
        active: &[bool],
        return_edge_original_idx: usize,
    ) -> (usize, usize, Vec<usize>, Vec<usize>, Vec<f64>, Vec<f64>, usize) {
        let mut node_used = vec![false; n_orig];
        for (idx, &(u, v, _)) in edges.iter().enumerate() {
            if active[idx] {
                node_used[u] = true;
                node_used[v] = true;
            }
        }
        let mut node_map = vec![0usize; n_orig];
        let mut new_n = 0;
        for v in 0..n_orig {
            if node_used[v] {
                node_map[v] = new_n;
                new_n += 1;
            }
        }

        let mut edge_from = Vec::new();
        let mut edge_to = Vec::new();
        let mut cap = Vec::new();
        let mut cost = Vec::new();
        let mut return_idx = 0;

        for (idx, &(u, v, c)) in edges.iter().enumerate() {
            if !active[idx] {
                continue;
            }
            if idx == return_edge_original_idx {
                return_idx = edge_from.len();
            }
            edge_from.push(node_map[u]);
            edge_to.push(node_map[v]);
            cap.push(c);
            cost.push(if idx == return_edge_original_idx {
                -1.0
            } else {
                0.0
            });
        }

        (new_n, edge_from.len(), edge_from, edge_to, cap, cost, return_idx)
    }

    // ========================================================================
    // Component 4: Excess computation
    // ========================================================================

    /// Compute flow conservation excess at each node.
    /// excess[v] = Σ f_out(v) - Σ f_in(v). Zero means conservation holds.
    pub(crate) fn compute_excess(
        n: usize,
        edge_from: &[usize],
        edge_to: &[usize],
        flow: &[f64],
    ) -> Vec<f64> {
        let mut excess = vec![0.0_f64; n];
        for i in 0..edge_from.len() {
            excess[edge_from[i]] += flow[i];
            excess[edge_to[i]] -= flow[i];
        }
        excess
    }

    // ========================================================================
    // Component 5: Step size computation
    // ========================================================================

    /// Compute the maximum step size α such that flow + α*df stays in (EPS, cap-EPS).
    /// Returns α * 0.99 (safety factor), clamped to [0, 1].
    pub(crate) fn compute_step_size(flow: &[f64], cap: &[f64], df: &[f64]) -> f64 {
        let mut alpha = 1.0_f64;
        for i in 0..flow.len() {
            if df[i] > 1e-15 {
                let bound = (cap[i] - EPS - flow[i]) / df[i];
                if bound < alpha {
                    alpha = bound;
                }
            } else if df[i] < -1e-15 {
                let bound = (flow[i] - EPS) / (-df[i]);
                if bound < alpha {
                    alpha = bound;
                }
            }
        }
        (alpha * 0.99).min(1.0)
    }

    // ========================================================================
    // Component 6: Flow initialization via weighted projection
    // ========================================================================

    /// Find a strictly interior feasible circulation by starting at f = cap/2
    /// and iteratively projecting onto the conservation constraints using
    /// a weighted Laplacian solve.
    ///
    /// The weight w_e = f_e * (cap_e - f_e) ensures corrections are small
    /// for edges near the boundary, preventing constraint violations.
    pub(crate) fn initialize_flow(
        n: usize,
        edge_from: &[usize],
        edge_to: &[usize],
        cap: &[f64],
    ) -> Vec<f64> {
        let m = edge_from.len();
        let mut flow: Vec<f64> = (0..m).map(|i| cap[i] / 2.0).collect();

        for _ in 0..100 {
            let excess = compute_excess(n, edge_from, edge_to, &flow);
            let max_excess = excess.iter().map(|x| x.abs()).fold(0.0_f64, f64::max);
            if max_excess < 1e-12 {
                break;
            }

            let neg_excess: Vec<f64> = excess.iter().map(|x| -x).collect();
            let cond: Vec<f64> = (0..m)
                .map(|i| flow[i].max(EPS) * (cap[i] - flow[i]).max(EPS))
                .collect();
            let pot = solve_laplacian(n, edge_from, edge_to, &cond, &neg_excess);
            let df: Vec<f64> = (0..m)
                .map(|i| (pot[edge_from[i]] - pot[edge_to[i]]) * cond[i])
                .collect();

            let mut alpha = compute_step_size(&flow, cap, &df);
            alpha = alpha.min(0.95); // conservative for initialization

            for i in 0..m {
                flow[i] += alpha * df[i];
                flow[i] = flow[i].max(EPS).min(cap[i] - EPS);
            }
        }

        flow
    }

    // ========================================================================
    // Component 7: Single IPM Newton step
    // ========================================================================

    /// Perform one infeasible primal-dual interior point Newton step.
    ///
    /// Computes the Newton direction by solving a weighted Laplacian system
    /// derived from the KKT conditions of the barrier-penalized min-cost
    /// circulation problem, then takes a step with line search.
    ///
    /// Returns the step size α taken.
    pub(crate) fn ipm_step(
        n: usize,
        edge_from: &[usize],
        edge_to: &[usize],
        cap: &[f64],
        cost: &[f64],
        flow: &mut [f64],
        potential: &mut [f64],
        mu: f64,
    ) -> f64 {
        let m = edge_from.len();

        // Primal residual (conservation violation)
        let p = compute_excess(n, edge_from, edge_to, flow);

        // Per-edge: residual r_e, Hessian h_e, inverse Hessian
        let mut r = vec![0.0_f64; m];
        let mut inv_h = vec![0.0_f64; m];
        let mut h = vec![0.0_f64; m];
        for i in 0..m {
            let s_e = flow[i];
            let t_e = cap[i] - flow[i];
            r[i] = cost[i] - mu / s_e + mu / t_e - potential[edge_from[i]] + potential[edge_to[i]];
            h[i] = mu / (s_e * s_e) + mu / (t_e * t_e);
            inv_h[i] = 1.0 / h[i];
        }

        // RHS = B H^{-1} r - p
        let mut rhs = vec![0.0_f64; n];
        for i in 0..m {
            let rh = r[i] * inv_h[i];
            rhs[edge_from[i]] += rh;
            rhs[edge_to[i]] -= rh;
        }
        for v in 0..n {
            rhs[v] -= p[v];
        }

        let dy = solve_laplacian(n, edge_from, edge_to, &inv_h, &rhs);

        // Flow update: Δf_e = (-r_e + Δy[from] - Δy[to]) / h_e
        let df: Vec<f64> = (0..m)
            .map(|i| (-r[i] + dy[edge_from[i]] - dy[edge_to[i]]) / h[i])
            .collect();

        let alpha = compute_step_size(flow, cap, &df);

        for i in 0..m {
            flow[i] += alpha * df[i];
        }
        for v in 0..n {
            potential[v] += alpha * dy[v];
        }

        alpha
    }
}

#[cfg(test)]
mod tests {
    use super::chen_flow::*;

    // ====================================================================
    // Unit tests for Component 1: solve_laplacian
    // ====================================================================

    #[test]
    fn test_laplacian_path_graph() {
        // Path graph: 0 -- 1 -- 2, all weights 1
        // Laplacian: [1 -1 0; -1 2 -1; 0 -1 1]
        // Solve L*x = [1, 0, -1] with x[0]=0
        // Expected: x = [0, 1, 1] (up to null space)
        let edge_from = vec![0, 1];
        let edge_to = vec![1, 2];
        let coeff = vec![1.0, 1.0];
        let rhs = vec![1.0, 0.0, -1.0];

        let x = solve_laplacian(3, &edge_from, &edge_to, &coeff, &rhs);
        assert_eq!(x[0], 0.0);

        // Verify L*x ≈ rhs
        let residual = laplacian_multiply(3, &edge_from, &edge_to, &coeff, &x, &rhs);
        for v in 0..3 {
            assert!(residual[v].abs() < 1e-10, "residual[{}] = {}", v, residual[v]);
        }
    }

    #[test]
    fn test_laplacian_triangle() {
        // Triangle: 0-1, 1-2, 0-2, all weights 1
        // Laplacian: [2 -1 -1; -1 2 -1; -1 -1 2]
        // Solve L*x = [2, -1, -1] with x[0]=0
        let edge_from = vec![0, 1, 0];
        let edge_to = vec![1, 2, 2];
        let coeff = vec![1.0, 1.0, 1.0];
        let rhs = vec![2.0, -1.0, -1.0];

        let x = solve_laplacian(3, &edge_from, &edge_to, &coeff, &rhs);
        assert_eq!(x[0], 0.0);

        let residual = laplacian_multiply(3, &edge_from, &edge_to, &coeff, &x, &rhs);
        for v in 0..3 {
            assert!(residual[v].abs() < 1e-10, "residual[{}] = {}", v, residual[v]);
        }
    }

    #[test]
    fn test_laplacian_weighted() {
        // Path: 0 --w=3-- 1 --w=5-- 2
        // L = [3 -3 0; -3 8 -5; 0 -5 5]
        // Solve L*x = [3, -8, 5] with x[0]=0
        let edge_from = vec![0, 1];
        let edge_to = vec![1, 2];
        let coeff = vec![3.0, 5.0];
        let rhs = vec![3.0, -8.0, 5.0];

        let x = solve_laplacian(3, &edge_from, &edge_to, &coeff, &rhs);

        let residual = laplacian_multiply(3, &edge_from, &edge_to, &coeff, &x, &rhs);
        for v in 0..3 {
            assert!(residual[v].abs() < 1e-10, "residual[{}] = {}", v, residual[v]);
        }
    }

    #[test]
    fn test_laplacian_single_node() {
        let x = solve_laplacian(1, &[], &[], &[], &[0.0]);
        assert_eq!(x, vec![0.0]);
    }

    // ====================================================================
    // Unit tests for Component 2: prune_dead_ends
    // ====================================================================

    #[test]
    fn test_prune_preserves_cycle() {
        // Cycle: 0->1->2->0, all should remain active
        let edges = vec![(0, 1, 1.0), (1, 2, 1.0), (2, 0, 1.0)];
        let active = prune_dead_ends(3, &edges);
        assert_eq!(active, vec![true, true, true]);
    }

    #[test]
    fn test_prune_removes_dead_end() {
        // 0->1->2, 2->0 (cycle), plus 0->3 (dead end, 3 has no outgoing)
        let edges = vec![(0, 1, 1.0), (1, 2, 1.0), (2, 0, 1.0), (0, 3, 1.0)];
        let active = prune_dead_ends(4, &edges);
        assert_eq!(active, vec![true, true, true, false]);
    }

    #[test]
    fn test_prune_cascading() {
        // 0->1->2->3, 3->0 (cycle), plus 4->1 (4 has no incoming, cascading)
        let edges = vec![
            (0, 1, 1.0),
            (1, 2, 1.0),
            (2, 3, 1.0),
            (3, 0, 1.0),
            (4, 1, 1.0),
        ];
        let active = prune_dead_ends(5, &edges);
        assert!(active[0] && active[1] && active[2] && active[3]);
        assert!(!active[4]); // 4->1 pruned because 4 has no incoming
    }

    #[test]
    fn test_prune_all_dead() {
        // 0->1->2 (no cycle, all should be pruned)
        let edges = vec![(0, 1, 1.0), (1, 2, 1.0)];
        let active = prune_dead_ends(3, &edges);
        assert_eq!(active, vec![false, false]);
    }

    // ====================================================================
    // Unit tests for Component 4: compute_excess
    // ====================================================================

    #[test]
    fn test_excess_zero_for_circulation() {
        // Cycle 0->1->2->0 with equal flow
        let edge_from = vec![0, 1, 2];
        let edge_to = vec![1, 2, 0];
        let flow = vec![5.0, 5.0, 5.0];
        let excess = compute_excess(3, &edge_from, &edge_to, &flow);
        for v in 0..3 {
            assert!(excess[v].abs() < 1e-10);
        }
    }

    #[test]
    fn test_excess_nonzero() {
        // 0->1 with flow 3, 1->2 with flow 1
        let edge_from = vec![0, 1];
        let edge_to = vec![1, 2];
        let flow = vec![3.0, 1.0];
        let excess = compute_excess(3, &edge_from, &edge_to, &flow);
        assert!((excess[0] - 3.0).abs() < 1e-10);
        assert!((excess[1] - (-2.0)).abs() < 1e-10);
        assert!((excess[2] - (-1.0)).abs() < 1e-10);
    }

    // ====================================================================
    // Unit tests for Component 5: compute_step_size
    // ====================================================================

    #[test]
    fn test_step_size_unbounded() {
        // flow well within bounds, small df
        let flow = vec![5.0, 5.0];
        let cap = vec![10.0, 10.0];
        let df = vec![0.1, -0.1];
        let alpha = compute_step_size(&flow, &cap, &df);
        assert!((alpha - 0.99).abs() < 1e-10); // full step
    }

    #[test]
    fn test_step_size_bounded() {
        // flow near upper bound, positive df
        let flow = vec![9.0];
        let cap = vec![10.0];
        let df = vec![2.0];
        // bound = (10 - eps - 9) / 2 ≈ 0.5
        let alpha = compute_step_size(&flow, &cap, &df);
        assert!(alpha < 0.5);
        assert!(alpha > 0.0);
        // Verify flow stays in bounds
        assert!(flow[0] + alpha * df[0] < cap[0]);
    }

    #[test]
    fn test_step_size_bounded_lower() {
        // flow near lower bound, negative df
        let flow = vec![0.5];
        let cap = vec![10.0];
        let df = vec![-2.0];
        let alpha = compute_step_size(&flow, &cap, &df);
        assert!(flow[0] + alpha * df[0] > 0.0);
    }

    // ====================================================================
    // Unit tests for Component 6: initialize_flow
    // ====================================================================

    #[test]
    fn test_init_flow_conservation() {
        // Triangle: 0->1, 1->2, 2->0
        let edge_from = vec![0, 1, 2];
        let edge_to = vec![1, 2, 0];
        let cap = vec![10.0, 5.0, 8.0];

        let flow = initialize_flow(3, &edge_from, &edge_to, &cap);
        let excess = compute_excess(3, &edge_from, &edge_to, &flow);
        for v in 0..3 {
            assert!(
                excess[v].abs() < 1e-6,
                "conservation violated at node {}: excess = {}",
                v,
                excess[v]
            );
        }
    }

    #[test]
    fn test_init_flow_interiority() {
        // Verify all flows are strictly between 0 and cap
        let edge_from = vec![0, 1, 2, 0];
        let edge_to = vec![1, 2, 0, 2];
        let cap = vec![10.0, 5.0, 8.0, 3.0];

        let flow = initialize_flow(3, &edge_from, &edge_to, &cap);
        for i in 0..flow.len() {
            assert!(
                flow[i] > 0.0,
                "flow[{}] = {} not > 0",
                i,
                flow[i]
            );
            assert!(
                flow[i] < cap[i],
                "flow[{}] = {} not < cap {}",
                i,
                flow[i],
                cap[i]
            );
        }
    }

    #[test]
    fn test_init_flow_larger_graph() {
        // 5 nodes, 7 edges forming multiple cycles
        let edge_from = vec![0, 0, 1, 1, 2, 3, 4];
        let edge_to = vec![1, 2, 2, 3, 4, 4, 0];
        let cap = vec![10.0, 5.0, 8.0, 3.0, 7.0, 4.0, 20.0];

        let flow = initialize_flow(5, &edge_from, &edge_to, &cap);

        // Check conservation
        let excess = compute_excess(5, &edge_from, &edge_to, &flow);
        for v in 0..5 {
            assert!(
                excess[v].abs() < 1e-6,
                "conservation violated at node {}: excess = {}",
                v,
                excess[v]
            );
        }

        // Check interiority
        for i in 0..flow.len() {
            assert!(flow[i] > 0.0 && flow[i] < cap[i]);
        }
    }

    // ====================================================================
    // Unit tests for Component 7: ipm_step
    // ====================================================================

    #[test]
    fn test_ipm_step_feasibility_preserved() {
        // After one IPM step, flow should still be in (0, cap)
        let edge_from = vec![0, 1, 2];
        let edge_to = vec![1, 2, 0];
        let cap = vec![10.0, 5.0, 8.0];
        let cost = vec![0.0, 0.0, -1.0];

        let mut flow = initialize_flow(3, &edge_from, &edge_to, &cap);
        let mut potential = vec![0.0; 3];

        ipm_step(3, &edge_from, &edge_to, &cap, &cost, &mut flow, &mut potential, 1.0);

        for i in 0..flow.len() {
            assert!(
                flow[i] > 0.0 && flow[i] < cap[i],
                "flow[{}] = {} out of bounds (0, {})",
                i,
                flow[i],
                cap[i]
            );
        }
    }

    #[test]
    fn test_ipm_step_positive_alpha() {
        // IPM step should take a positive step
        let edge_from = vec![0, 1, 2];
        let edge_to = vec![1, 2, 0];
        let cap = vec![10.0, 5.0, 8.0];
        let cost = vec![0.0, 0.0, -1.0];

        let mut flow = initialize_flow(3, &edge_from, &edge_to, &cap);
        let mut potential = vec![0.0; 3];

        let alpha = ipm_step(
            3, &edge_from, &edge_to, &cap, &cost, &mut flow, &mut potential, 1.0,
        );
        assert!(alpha > 0.0, "alpha should be positive, got {}", alpha);
    }

    #[test]
    fn test_ipm_convergence_simple() {
        // Run multiple IPM steps on a simple max-flow instance
        // Graph: 0->1 (cap 5), 0->2 (cap 3), 1->3 (cap 4), 2->3 (cap 6)
        // Plus return edge 3->0 (cap 13, cost -1)
        // Max flow should be 5+3 = 8 (limited by source out-capacity)
        // Actually max flow = min(5+3, 4+6) = 8
        let edge_from = vec![0, 0, 1, 2, 3];
        let edge_to = vec![1, 2, 3, 3, 0];
        let cap = vec![5.0, 3.0, 4.0, 6.0, 13.0];
        let cost = vec![0.0, 0.0, 0.0, 0.0, -1.0];

        let mut flow = initialize_flow(4, &edge_from, &edge_to, &cap);
        let mut potential = vec![0.0; 4];
        let mut mu = 1.0;
        let shrink = 1.0 - 0.4 / (5.0_f64).sqrt();

        for _ in 0..5000 {
            if mu < 1e-9 {
                break;
            }
            ipm_step(
                4, &edge_from, &edge_to, &cap, &cost, &mut flow, &mut potential, mu,
            );
            mu *= shrink;
        }

        let result = flow[4].round() as i64; // return edge
        assert_eq!(result, 7, "expected max flow 7, got {}", result);
    }

    // ====================================================================
    // Integration test
    // ====================================================================

    #[test]
    fn test_max_flow_simple() {
        let mut f = ChenFlow::new(4);
        f.add_edge(0, 1, 2);
        f.add_edge(0, 2, 1);
        f.add_edge(1, 2, 1);
        f.add_edge(1, 3, 1);
        f.add_edge(2, 3, 2);
        assert_eq!(f.max_flow(0, 3), 3);
    }

    #[test]
    fn test_max_flow_no_path() {
        let mut f = ChenFlow::new(4);
        f.add_edge(0, 1, 5);
        f.add_edge(2, 3, 5);
        assert_eq!(f.max_flow(0, 3), 0);
    }

    #[test]
    fn test_max_flow_single_edge() {
        let mut f = ChenFlow::new(2);
        f.add_edge(0, 1, 42);
        assert_eq!(f.max_flow(0, 1), 42);
    }

    #[test]
    fn solve_grl_6_a() {
        use crate::utils::test_helper::Tester;
        let tester = Tester::new("./assets/GRL_6_A/in/", "./assets/GRL_6_A/out/");
        tester.test_solution(|sc| {
            let v: usize = sc.read();
            let e: usize = sc.read();
            let mut flow = ChenFlow::new(v);
            for _ in 0..e {
                let from: usize = sc.read();
                let to: usize = sc.read();
                let c: i64 = sc.read();
                flow.add_edge(from, to, c);
            }
            let result = flow.max_flow(0, v - 1);
            sc.write(format!("{}\n", result));
        });
    }

    // ====================================================================
    // Helper for Laplacian tests
    // ====================================================================

    /// Compute L*x - rhs (residual of the Laplacian system)
    fn laplacian_multiply(
        n: usize,
        edge_from: &[usize],
        edge_to: &[usize],
        coeff: &[f64],
        x: &[f64],
        rhs: &[f64],
    ) -> Vec<f64> {
        let mut result = vec![0.0; n];
        for e in 0..edge_from.len() {
            let u = edge_from[e];
            let v = edge_to[e];
            let w = coeff[e];
            let diff = x[u] - x[v];
            result[u] += w * diff;
            result[v] -= w * diff;
        }
        for v in 0..n {
            result[v] -= rhs[v];
        }
        result
    }
}

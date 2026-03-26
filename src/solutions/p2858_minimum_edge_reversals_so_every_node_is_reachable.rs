#![allow(dead_code)]
#![allow(clippy::doc_lazy_continuation)]

/// Problem: Minimum Edge Reversals So Every Node Is Reachable
/// Difficulty: Hard
/// URL: https://leetcode.com/problems/minimum-edge-reversals-so-every-node-is-reachable/
///
/// There is a **simple directed graph** with `n` nodes labeled from `0` to `n - 1`. The graph would
/// form a **tree** if its edges were bi-directional.
/// You are given an integer `n` and a **2D** integer array `edges`, where `edges[i] = [u_i, v_i]`
/// represents a **directed edge** going from node `u_i` to node `v_i`.
/// An **edge reversal** changes the direction of an edge, i.e., a directed edge going from node
/// `u_i` to node `v_i` becomes a directed edge going from node `v_i` to node `u_i`.
/// For every node `i` in the range `[0, n - 1]`, your task is to **independently** calculate the
/// **minimum** number of **edge reversals** required so it is possible to reach any other node
/// starting from node `i` through a **sequence** of **directed edges**.
/// Return *an integer array *`answer`*, where *`answer[i]`* is the******minimum** number of **edge
/// reversals** required so it is possible to reach any other node starting from node *`i`* through
/// a **sequence** of **directed edges**.*
/// **Example 1:**
/// '''
/// **Input:** n = 4, edges = [[2,0],[2,1],[1,3]] **Output:** [1,1,0,2]
/// '''
/// **Example 2:**
/// '''
/// **Input:** n = 3, edges = [[1,2],[2,0]] **Output:** [2,0,1]
/// '''
/// **Constraints:**
/// - `2 <= n <= 10^5`
/// - `edges.length == n - 1`
/// - `edges[i].length == 2`
/// - `0 <= u_i == edges[i][0] < n`
/// - `0 <= v_i == edges[i][1] < n`
/// - `u_i != v_i`
/// - The input is generated such that if the edges were bi-directional, the graph would be a tree.
pub struct Solution;

// Solved by Kimi atomically
impl Solution {
    pub fn min_edge_reversals(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        // Build adjacency list
        // (neighbor, needs_reversal)
        // needs_reversal = 1 if we need to reverse the edge to go from u to neighbor
        let mut adj: Vec<Vec<(usize, i32)>> = vec![vec![]; n];

        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            // u -> v (original direction)
            // From u to v: no reversal needed (0)
            // From v to u: reversal needed (1)
            adj[u].push((v, 0));
            adj[v].push((u, 1));
        }

        // First DFS: calculate reversals needed from node 0
        // dp[u] = total reversals needed in the tree rooted at u
        let mut dp = vec![0; n];

        fn dfs1(node: usize, parent: usize, adj: &Vec<Vec<(usize, i32)>>, dp: &mut Vec<i32>) {
            for &(next, cost) in &adj[node] {
                if next != parent {
                    dfs1(next, node, adj, dp);
                    dp[node] += dp[next] + cost;
                }
            }
        }

        dfs1(0, n, &adj, &mut dp);

        // Second DFS: reroot to calculate for all nodes
        let mut answer = dp.clone();

        fn dfs2(node: usize, parent: usize, adj: &Vec<Vec<(usize, i32)>>, answer: &mut Vec<i32>) {
            for &(next, cost) in &adj[node] {
                if next != parent {
                    // When moving from node to next:
                    // - If edge is node -> next (cost 0): From node: we didn't count this as
                    //   reversal From next: we need to reverse it to come back, so +1 Change: +1
                    // - If edge is next -> node (cost 1): From node: we counted this as reversal
                    //   From next: we don't need to reverse it, so -1 Change: -1
                    // Formula: answer[next] = answer[node] + 1 - 2*cost
                    answer[next] = answer[node] + 1 - 2 * cost;
                    dfs2(next, node, adj, answer);
                }
            }
        }

        dfs2(0, n, &adj, &mut answer);

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_3105_1() {
        let n = 4;
        let edges = vec![vec![2, 0], vec![2, 1], vec![1, 3]];
        assert_eq!(Solution::min_edge_reversals(n, edges), vec![1, 1, 0, 2]);
    }

    #[test]
    fn test_case_3105_2() {
        let n = 3;
        let edges = vec![vec![1, 2], vec![2, 0]];
        assert_eq!(Solution::min_edge_reversals(n, edges), vec![2, 0, 1]);
    }
}

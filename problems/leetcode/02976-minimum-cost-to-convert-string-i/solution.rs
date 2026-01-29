use std::cmp::Reverse;
use std::collections::BinaryHeap;

const ALPHABET_SIZE: usize = 26;
const INF: usize = usize::MAX;

impl Solution {
    fn char_to_id(id: char) -> usize {
        return (id as u8 - b'a') as usize;
    }

    // Compute shortest paths by using Dijkstra Algorithm
    fn compute_shortest_paths(
        adj_list: &[Vec<(usize, usize)>],
        start_node: usize,
        distances: &mut [usize],
    ) {
        distances[start_node] = 0;

        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, start_node)));

        while let Some(Reverse((d, u))) = pq.pop() {
            if d > distances[u] {
                continue;
            }

            for &(v, weight) in &adj_list[u] {
                let next_dist = d + weight;

                if next_dist < distances[v] {
                    distances[v] = next_dist;
                    pq.push(Reverse((next_dist, v)));
                }
            }
        }
    }

    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        // Build Adjacency List
        let mut adj_list = vec![Vec::new(); ALPHABET_SIZE];

        for i in 0..original.len() {
            let u = Self::char_to_id(original[i]);
            let v = Self::char_to_id(changed[i]);
            let w = cost[i] as usize;

            adj_list[u].push((v, w));
        }

        // Pre-compute all pairs shortest paths.
        let mut min_costs = [[usize::MAX; ALPHABET_SIZE]; ALPHABET_SIZE];
        for i in 0..ALPHABET_SIZE {
            Self::compute_shortest_paths(&adj_list, i, &mut min_costs[i]);
        }

        let mut total_cost: usize = 0;
        for (s_char, t_char) in source.chars().zip(target.chars()) {
            if s_char == t_char {
                continue;
            }

            let src = Self::char_to_id(s_char);
            let dst = Self::char_to_id(t_char);

            let cost_for_change = min_costs[src][dst];

            if cost_for_change == INF {
                return -1;
            }
            total_cost += cost_for_change;
        }
        i64::try_from(total_cost).unwrap_or(-1)
    }
}

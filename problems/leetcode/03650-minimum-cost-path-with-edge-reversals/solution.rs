use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![Vec::new(); n];

        for edge in &edges {
            if let &[u, v, w] = &edge[..] {
                adj[u as usize].push((v as usize, w));
                adj[v as usize].push((u as usize, w * 2));
            }
        }

        let mut dist = vec![i32::MAX; n];
        let (SOURCE, DESTINATION) = (0 as usize, (n - 1) as usize);
        dist[SOURCE] = 0;

        let mut pq = BinaryHeap::new();
        pq.push(Reverse((dist[SOURCE], SOURCE)));

        while let Some(Reverse((d, u))) = pq.pop() {
            if u == DESTINATION {
                return dist[u];
            }

            if d > dist[u] {
                continue;
            }

            for &(v, w) in &adj[u] {
                let new_dist = d.saturating_add(w);
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    pq.push(Reverse((dist[v], v)));
                }
            }
        }

        return -1;
    }
}

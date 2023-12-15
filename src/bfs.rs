use std::collections::VecDeque;
pub use crate::readfile::Graph;

pub type Vertex = usize;

#[derive(Debug)]
pub struct ShortestPathResult {
// struct that holds distance/degree from a node to each other node.
    pub distance: Vec<u32>,
}

impl Graph {
    pub fn compute_bfs(&self, start: Vertex) -> ShortestPathResult {
    // function computes the Breadth-First Search algorithm and returns the shortest path distances from a starting vertex to 
    // all other vertices in the graph.
    let mut distance: Vec<Option<u32>> = vec![None; self.n];

    distance[start] = Some(0);
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
        while let Some(v) = queue.pop_front() {
            for &u in self.outedges.get(&v).unwrap_or(&vec![]).iter() {
                if distance[u].is_none() {
                    distance[u] = Some(distance[v].unwrap() + 1);
                    queue.push_back(u);
                }
            }
        } 
        let distance_as_number: Vec<u32> = distance.into_iter().map(|d| d.unwrap_or(0)).collect();

        ShortestPathResult { distance: distance_as_number }
    }
}
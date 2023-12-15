use crate::bfs::Graph;
use std::collections::HashSet;

pub fn recommendations(graph: &Graph, start_vertex: usize, distance: usize) -> HashSet<usize> {
    // This function uses Breadth-First Search to find friend connections a specified distance away from the user/start vertex. 
    // The result is a HashSet containing vertex identifiers of recommended friends.
    let bfs_result = graph.compute_bfs(start_vertex);

    let recommended_friends: HashSet<usize> = bfs_result.distance.iter().enumerate().filter_map(|(vertex, &dist)| {
            if distance == dist as usize && vertex != start_vertex {
                Some(vertex)
            } else {
                None
            }
        }).collect();

    recommended_friends
}
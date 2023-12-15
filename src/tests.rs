#[cfg(test)]
mod tests {
    use crate::readfile::Graph;
    use std::collections::{HashMap,HashSet};
    use crate::friendrecs::recommendations;


    #[test]
    fn test_bfs_compute() {
        let start_vertex = 0;

        let mut graph = Graph {
            n: 6,
            outedges: HashMap::new(),
        };
        graph.outedges.insert(0, vec![1, 2]);
        graph.outedges.insert(1, vec![3, 4]);
        graph.outedges.insert(2, vec![5]);

        let result = graph.compute_bfs(start_vertex);

        let expected_result = vec![0,1,1,2,2,2];

        assert_eq!(result.distance,expected_result);
        
    }

    #[test]
    fn test_recommendations() {
        let start_vertex = 0;
        let distance = 2;

        let mut graph = Graph {
            n: 6,
            outedges: HashMap::new(),
        };
        graph.outedges.insert(0, vec![1, 2]);
        graph.outedges.insert(1, vec![3, 4]);
        graph.outedges.insert(2, vec![5]);

        let result = recommendations(&graph, start_vertex, distance);

        let expected_result: HashSet<usize> = vec![3,4,5].into_iter().collect();

        assert_eq!(result,expected_result);
        
    }
}
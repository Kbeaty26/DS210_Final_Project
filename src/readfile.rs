// This module contains the function and struct to read a inputed text file.
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};

pub type AdjacencyLists = HashMap<usize, Vec<usize>>;

#[derive(Debug)]
// This struct contains n (the number of vertices) and outedges (adjacency list for each vertex).
pub struct Graph {
    pub n: usize,
    pub outedges: AdjacencyLists,
}

impl Graph {
    pub fn new() -> Self {
        // function to create a new graph
        Self {
            n: 0,
            outedges: HashMap::new(),
        }
    }
    pub fn read_file(&mut self, path: &str) {
        // Function takes an inputed text file, then assigns n to the vertices and assigns a created adjacency list to outedges.
        let file = File::open(path).expect("Could not open file");
        let mut contents: Vec<(usize, usize)> = Vec::new();
        let buf_reader = io::BufReader::new(file).lines();
        for line in buf_reader {
            let line_str = line.expect("Error reading");
            let v: Vec<&str> = line_str.trim().split(',').collect();
            let labels = v[0].parse::<usize>().unwrap();
            let edges = v[1].parse::<usize>().unwrap();
            contents.push((labels, edges));
        }
        // populates the adjacency list and assings it to outedges by iterating over a collection of edges.
        for &(from, to) in &contents {
            // Add both (A, B) and (B, A) for an undirected graph
            self.outedges.entry(from).or_insert(vec![]).push(to);
            self.outedges.entry(to).or_insert(vec![]).push(from);
        }
        // assigns n to the max node number as vertices can repeat themselves in the text file.
        self.n = contents.iter().map(|&(a, b)| a.max(b)).max().unwrap_or(0) + 1;
    }       
}
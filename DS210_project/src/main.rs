use petgraph::graph::Graph;
use petgraph::algo::dijkstra;
use std::collections::HashMap;
use std::error::Error;


// Build a graph of asteroids based on proximity
fn build_graph(data: &Vec<csv_reader::AsteroidData>, threshold: f64) -> Graph<String, f64> {
    let mut graph = Graph::<String, f64>::new();
    let mut node_map = HashMap::new();

    // Add nodes to the graph
    for record in data {
        let node = graph.add_node(record.des.clone());
        node_map.insert(record.des.clone(), node);
    }
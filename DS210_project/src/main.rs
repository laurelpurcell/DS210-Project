use petgraph::graph::Graph;
use petgraph::algo::dijkstra;
use std::collections::HashMap;
use std::error::Error;


// builds a graph of the asteroids in the dataset based on proximity to earth
fn build_graph(data: &Vec<csv_reader::AsteroidData>, threshold: f64) -> Graph<String, f64> {
    let mut graph = Graph::<String, f64>::new();
    let mut node_map = HashMap::new();

    // add nodes to the graph
    for record in data {
        let node = graph.add_node(record.des.clone());
        node_map.insert(record.des.clone(), node);
    }

    // add edges between close approaches
    for (i, a) in data.iter().enumerate() {
        for (_j, b) in data.iter().enumerate().skip(i + 1) {
            if a.dist.is_nan() || b.dist.is_nan() {
                continue; // skips invalid rows
            }
            let distance = (a.dist - b.dist).abs();
            if distance <= threshold {
                graph.add_edge(node_map[&a.des], node_map[&b.des], distance);
            }
        }
    }

    graph
}

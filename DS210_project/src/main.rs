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

// compute the centrality measures (closeness centrality)
fn compute_centrality(graph: &Graph<String, f64>) -> HashMap<String, f64> {
    let mut centrality = HashMap::new();

    for node in graph.node_indices() {
        let distances = dijkstra(graph, node, None, |e| *e.weight());
        let total_distance: f64 = distances.values().sum();
        let closeness = if total_distance > 0.0 {
            1.0 / total_distance
        } else {
            0.0
        };
        let node_name = &graph[node];
        centrality.insert(node_name.clone(), closeness);
    }

    centrality
}

// function to cluster asteroids by proximity using connected components
fn cluster_asteroids(graph: &Graph<String, f64>) -> Vec<Vec<String>> {
    let mut clusters = Vec::new();
    let mut visited = HashMap::new();

    for node in graph.node_indices() {
        visited.insert(node, false);
    }

    for node in graph.node_indices() {
        if !visited[&node] {
            let mut cluster = Vec::new();
            let mut stack = vec![node];

            while let Some(current) = stack.pop() {
                if visited[&current] {
                    continue;
                }

                visited.insert(current, true);
                cluster.push(graph[current].clone());

                for neighbor in graph.neighbors(current) {
                    if !visited[&neighbor] {
                        stack.push(neighbor);
                    }
                }
            }

            clusters.push(cluster);
        }
    }

    clusters
}

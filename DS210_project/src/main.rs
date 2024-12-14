use petgraph::graph::Graph;
use petgraph::algo::dijkstra;
use std::collections::HashMap;
use std::error::Error;
mod csv_reader;
use csv_reader::read_csv;

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

// find the top 50 asteroids that passed closest to Earth
fn top_closest_asteroids(data: &Vec<csv_reader::AsteroidData>) -> Vec<(String, f64)> {
    let mut closest = data
        .iter()
        .map(|a| (a.des.clone(), a.dist))
        .collect::<Vec<_>>();

    closest.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    closest.into_iter().take(50).collect()
}
fn main() -> Result<(), Box<dyn Error>> {
    // specify the file path to the CSV dataset
    let file_path = "/Users/laurelpurcell/Downloads/DS210_asteroid_data.csv";

    // read the CSV file using the csv_reader module
    let data: Vec<csv_reader::AsteroidData> = read_csv(file_path.to_string())?;

    // define a proximity threshold (in AU)
    let threshold = 0.01;

    // guild the graph
    let graph = build_graph(&data, threshold);

    // compute centrality measures
    let _centrality = compute_centrality(&graph);

    // cluster asteroids 
    let _clusters = cluster_asteroids(&graph);

    // print top 50 closest asteroids
    println!("\nTop 50 Closest Asteroids:");
    let closest_asteroids = top_closest_asteroids(&data);
    for (name, dist) in closest_asteroids {
        println!("{}: {:.6} AU", name, dist);
    }

    Ok(())
}

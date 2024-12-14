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


#[test]
fn test_top_50_closest_asteroids() {
    let data = vec![ // creates vectors with example data to test
        csv_reader::AsteroidData {
            des: "Asteroid A".to_string(),
            orbit_id: "1".to_string(),
            jd: 2459200.5,
            cd: "2021-01-01".to_string(),
            dist: 0.002,
            dist_min: 0.0019,
            dist_max: 0.0021,
            v_rel: 5.0,
            v_inf: 4.8,
            t_sigma_f: "0".to_string(),
        },
        csv_reader::AsteroidData {
            des: "Asteroid B".to_string(),
            orbit_id: "2".to_string(),
            jd: 2459201.5,
            cd: "2021-01-02".to_string(),
            dist: 0.003,
            dist_min: 0.0029,
            dist_max: 0.0031,
            v_rel: 6.0,
            v_inf: 5.9,
            t_sigma_f: "0".to_string(),
        },
        csv_reader::AsteroidData {
            des: "Asteroid C".to_string(),
            orbit_id: "3".to_string(),
            jd: 2459202.5,
            cd: "2021-01-03".to_string(),
            dist: 0.001,
            dist_min: 0.0009,
            dist_max: 0.0011,
            v_rel: 7.0,
            v_inf: 6.8,
            t_sigma_f: "0".to_string(),
        },
    ];

    let result = top_closest_asteroids(&data);

    assert_eq!(result.len(), 3); // fewer than 50 in the dataset
    assert_eq!(result[0].0, "Asteroid C");
    assert_eq!(result[0].1, 0.001);
    assert_eq!(result[1].0, "Asteroid A");
    assert_eq!(result[1].1, 0.002);
    assert_eq!(result[2].0, "Asteroid B");
    assert_eq!(result[2].1, 0.003);
}



#[test]
fn test_building_threshold() {
    let data = vec![
        csv_reader::AsteroidData {
            des: "Asteroid X".to_string(),
            orbit_id: "10".to_string(),
            jd: 2459205.5,
            cd: "2021-01-05".to_string(),
            dist: 0.002,
            dist_min: 0.0019,
            dist_max: 0.0021,
            v_rel: 5.5,
            v_inf: 5.2,
            t_sigma_f: "0".to_string(),
        },
        csv_reader::AsteroidData {
            des: "Asteroid Y".to_string(),
            orbit_id: "11".to_string(),
            jd: 2459206.5,
            cd: "2021-01-06".to_string(),
            dist: 0.0025,
            dist_min: 0.0024,
            dist_max: 0.0026,
            v_rel: 6.5,
            v_inf: 6.2,
            t_sigma_f: "0".to_string(),
        },
        csv_reader::AsteroidData {
            des: "Asteroid Z".to_string(),
            orbit_id: "12".to_string(),
            jd: 2459207.5,
            cd: "2021-01-07".to_string(),
            dist: 0.01,
            dist_min: 0.009,
            dist_max: 0.011,
            v_rel: 7.5,
            v_inf: 7.2,
            t_sigma_f: "0".to_string(),
        },
    ];

    let threshold = 0.001; // threshold is too small, no edges expected
    let graph = build_graph(&data, threshold);

    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 0);

    let threshold = 0.01; // larger threshold, links should exist
    let graph = build_graph(&data, threshold);

    assert_eq!(graph.node_count(), 3);
    assert!(graph.edge_count() > 0); // ensures there are at least some edges
}


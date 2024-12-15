use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use std::error::Error;
mod csv_reader;
use csv_reader::read_csv;

// build a graph where nodes represent asteroids and edges indicate hazard comparisons
pub fn build_hazard_graph(data: &Vec<csv_reader::AsteroidData>, dist_threshold: f64, velocity_threshold: f64) -> Graph<(String, f64, f64), f64> {
    let mut graph = Graph::<(String, f64, f64), f64>::new();
    let mut node_map = HashMap::new();

    // add nodes to the graph
    for record in data {
        if record.dist_min <= dist_threshold && record.v_rel >= velocity_threshold {
            let node = graph.add_node((record.des.clone(), record.dist_min, record.v_rel));
            node_map.insert(record.des.clone(), node);
        }
    }

    // add edges to compare the hazard levels between asteroids
    for node_a in graph.node_indices() {
        for node_b in graph.node_indices() {
            if node_a == node_b {
                continue; // skip self-comparison
            }
            
            let asteroid_a = &graph[node_a];
            let asteroid_b = &graph[node_b];

            // hazard score comparison
            let hazard_a = asteroid_a.1 / asteroid_a.2; // Higher velocity & closer distance increases hazard
            let hazard_b = asteroid_b.1 / asteroid_b.2;

            // add edges weighted by difference in hazard score
            let weight = (hazard_a - hazard_b).abs();
            graph.add_edge(node_a, node_b, weight);
        }
    } // nodes are connected if both asteroids meet a certain "hazard threshold"

    graph
}

// rank asteroids based on hazard score and include details in the result
pub fn rank_hazardous_asteroids_with_details(data: &Vec<csv_reader::AsteroidData>) -> Vec<(String, f64, f64, String)> {
    let mut ranked_asteroids = data
        .iter()
        .map(|a| {
            let hazard_score = if a.dist_min > 0.0 {
                a.v_rel / a.dist_min // higher score for closer and faster asteroids
            } else {
                f64::INFINITY // assign very high hazard for zero distances
            };
            (a.des.clone(), hazard_score, a.dist_min, a.cd.clone())
        })
        .collect::<Vec<_>>();

    // sort by hazard score in descending order
    ranked_asteroids.sort_by(|a, b| {
        b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
    });

    ranked_asteroids
}

/// cluster asteroids based on hazard score
pub fn cluster_asteroids_by_hazard(data: &[(String, f64, f64, String)]) -> HashMap<String, Vec<(String, f64, f64, String)>> {
    // define hazard score thresholds for clusters
    let thresholds = vec![
        (0.0, 10.0, "Negligible Risk"),    // Scores between 0 and 10
        (10.0, 50.0, "Low Risk"), // Scores between 10 and 50
        (50.0, 100.0, "Moderate Risk"), // Scores between 50 and 100
        (100.0, f64::INFINITY, "Highest Risk"), // Scores above 100
    ];

    let mut clusters: HashMap<String, Vec<(String, f64, f64, String)>> = HashMap::new();

    // initialize clusters
    for (_, _, label) in &thresholds {
        clusters.insert(label.to_string(), Vec::new());
    }

    // assign each asteroid to a cluster based on hazard score
    for asteroid in data {
        let score = asteroid.1;
        for (min, max, label) in &thresholds {
            if score >= *min && score < *max {
                clusters.get_mut(&label.to_string()).unwrap().push(asteroid.clone());
                break;
            }
        }
    }

    clusters
}

fn main() -> Result<(), Box<dyn Error>> {
    // path to the CSV file
    let file_path = "/Users/laurelpurcell/Downloads/DS210_asteroid_data.csv".to_string();

    // check if file exists
    if !std::path::Path::new(&file_path).exists() {
        eprintln!("Error: File not found at path: {}", file_path);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")));
    }

    // read the CSV file with the csv_reader
    let data: Vec<csv_reader::AsteroidData> = read_csv(file_path)?;

    // rank the asteroids based on their hazard score
    let ranked_asteroids = rank_hazardous_asteroids_with_details(&data);

    // Cluster asteroids by hazard score
    let clusters = cluster_asteroids_by_hazard(&ranked_asteroids);

    // builds hazard graph
    let dist_threshold = 0.05;  // minimum distance (in AU)
    let velocity_threshold = 10.0; // minimum relative velocity (in km/s)
    let _hazard_graph = build_hazard_graph(&data, dist_threshold, velocity_threshold);

    // print the top 50 hazardous asteroids with their details
    println!("Top 50 Hazardous Asteroids:");
    println!("{:<25} {:<15} {:<15} {:<20} {:<15} {:<15}", "Asteroid", "Min Distance (AU)", "Velocity (km/s)", "Closest Approach Date", "Hazard Score", "Hazard Cluster");
    println!("{}", "-".repeat(105));
    for asteroid in ranked_asteroids.iter().take(50) {
        let cluster = clusters.iter().find(|(_, asteroids)| {
            asteroids.iter().any(|(n, _, _, _)| n == &asteroid.0)
        }).map(|(cluster_name, _)| cluster_name.clone()).unwrap_or("Unknown".to_string());

        println!("{:<25} {:<15.6} {:<15.2} {:<20} {:<15.2} {:<15}",
            asteroid.0, asteroid.2, asteroid.1, asteroid.3, asteroid.1 / asteroid.2, cluster);
    }

    // established an interactive lookup to manually search for asteroids
    use std::io::{self, Write};

    println!("\nEnter the name of an asteroid to retrieve details or type 'exit' to quit:");
    loop {
        print!("Asteroid name: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        if let Some(asteroid) = ranked_asteroids.iter().find(|(name, _, _, _)| name == input) {
            let cluster = clusters.iter().find(|(_, asteroids)| {
                asteroids.iter().any(|(n, _, _, _)| n == &asteroid.0)
            }).map(|(cluster_name, _)| cluster_name.clone()).unwrap_or("Unknown".to_string());

            println!("\nDetails for asteroid '{}':", asteroid.0);
            println!("{:<15}: {:.6} AU", "Min Distance", asteroid.2);
            println!("{:<15}: {:.2} km/s", "Velocity", asteroid.1);
            println!("{:<15}: {}", "Date", asteroid.3);
            println!("{:<15}: {}", "Hazard Cluster", cluster);
        } else {
            println!("Asteroid '{}' not found in the dataset.", input);
        }
    }

    Ok(())
}

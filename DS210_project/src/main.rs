use petgraph::graph::Graph;
use std::collections::HashMap;
use std::error::Error;
mod csv_reader;
use csv_reader::read_csv;

// build a graph where nodes represent asteroids and edges indicate hazard comparisons
pub fn build_hazard_graph(data: &Vec<csv_reader::AsteroidData>, dist_threshold: f64, velocity_threshold: f64) -> Graph<(String, f64, f64), f64> {
    let mut graph = Graph::<(String, f64, f64), f64>::new(); // new mutable graph
    let mut node_map = HashMap::new(); // new mutable HashMap

    // add nodes to the graph
    for record in data { // iterates through records
        if record.dist_min <= dist_threshold && record.v_rel >= velocity_threshold { // must meet the criteria for thresholds to make analysis more efficient
            let node = graph.add_node((record.des.clone(), record.dist_min, record.v_rel));
            node_map.insert(record.des.clone(), node);
        }
    }

    // add edges to compare hazard levels between asteroids
    for node_a in graph.node_indices() { // iterates through nodes
        for node_b in graph.node_indices() { // iterates through nodes
            if node_a == node_b { 
                continue; // do not compare to self 
            }

            let asteroid_a = &graph[node_a]; // defines node
            let asteroid_b = &graph[node_b]; // defines node

            // hazard score comparison
            let hazard_a = asteroid_a.1 / asteroid_a.2; // a higher velocity & closer distance increases hazard
            let hazard_b = asteroid_b.1 / asteroid_b.2; // does the same for node b 

            // add edges weighted by difference in hazard score
            let weight = (hazard_a - hazard_b).abs(); // absolute difference in hazard
            graph.add_edge(node_a, node_b, weight); // adds the edge based on the weight 
        }
    }
    // nodes are connected if both asteroids meet a certain "hazard threshold"
    graph
}

// rank asteroids based on hazard score and include details in the result
pub fn rank_hazardous_asteroids(data: &Vec<csv_reader::AsteroidData>) -> Vec<(String, f64, f64, String)> {
    let mut ranked_asteroids = data
        .iter()
        .map(|a| {
            let hazard_score = if a.dist_min > 0.0 { // ensures that the distance is not zero
                (a.v_rel / a.dist_min) / 1_000_000.0 // scale down hazard scores
            } else {
                f64::INFINITY // assign very high hazard for zero distances (as it would be on a collision path)
            };
            (a.des.clone(), hazard_score, a.dist_min, a.cd.clone()) // clones the score
        })
        .collect::<Vec<_>>(); // collects the outcome

    // sort by hazard score in descending order
    ranked_asteroids.sort_by(|a, b| { // sorts the hazards with a closure function 
        b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal) // floating-point comparison, with allowance for error handling
    });

    ranked_asteroids
}

// cluster the asteroids based on their hazard score
pub fn cluster_asteroids_by_hazard(data: &[(String, f64, f64, String)]) -> HashMap<String, Vec<(String, f64, f64, String)>> {
    // define hazard score thresholds for clusters
    let thresholds = vec![ // creates vector of score ranges and their label
        (0.0, 0.01, "Negligible Risk"),       // scores between 0 and 0.01
        (0.01, 0.05, "Low Risk"), // scores between 0.01 and 0.05
        (0.05, 0.1, "Moderate Risk"),      // scores between 0.05 and 0.1
        (0.1, f64::INFINITY, "Highest Risk"), // scores above 0.1
    ];

    let mut clusters: HashMap<String, Vec<(String, f64, f64, String)>> = HashMap::new();

    // initialize the clusters
    for (_, _, label) in &thresholds { 
        clusters.insert(label.to_string(), Vec::new()); 
    }

    // assign each asteroid to a cluster based on its hazard score
    for asteroid in data {
        let score = asteroid.1; // score of asteroid  1
        for (min, max, label) in &thresholds { // iterates over tuples in thresholds
            if score >= *min && score < *max { // compares the score to the max and min of the range
                clusters.get_mut(&label.to_string()).unwrap().push(asteroid.clone()); // adds score to cluster if it falls within the range
                break;
            }
        }
    }

    clusters
}

fn main() -> Result<(), Box<dyn Error>> {
    // define the path to the CSV file
    let file_path = "/Users/laurelpurcell/Downloads/DS210_asteroid_data.csv".to_string(); // local path

    // check if the file exists
    if !std::path::Path::new(&file_path).exists() {
        eprintln!("Error: File not found at path: {}", file_path);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))); // error if the file is not present
    }

    // read the CSV file
    let data: Vec<csv_reader::AsteroidData> = read_csv(file_path)?; // uses csv_reader

    // rank the asteroids based on their hazard score
    let ranked_asteroids = rank_hazardous_asteroids(&data); // calls ranked_asteroids function

    // cluster the asteroids by hazard score
    let clusters = cluster_asteroids_by_hazard(&ranked_asteroids); // calls clustering function

    // build the hazard graph
    let dist_threshold = 0.05;  // minimum distance (in AU)
    let velocity_threshold = 10.0; // minimum relative velocity (in km/s); this is a sufficiently fast speed
    let _hazard_graph = build_hazard_graph(&data, dist_threshold, velocity_threshold); // builds the hazard graph 

    // print the top 50 hazardous asteroids with their details
    println!("Top 50 Hazardous Asteroids:");
    println!("{:<25} {:<15} {:<15} {:<20} {:<15} {:<15}", "Asteroid", "Min Distance (AU)", "Velocity (km/s)", "Closest Approach Date", "Hazard Score", "Hazard Cluster");
    println!("{}", "-".repeat(105)); // accesses each of the necessary parts of the details
    for asteroid in ranked_asteroids.iter().take(50) {
        let cluster = clusters.iter().find(|(_, asteroids)| {
            asteroids.iter().any(|(n, _, _, _)| n == &asteroid.0) // iterates over the asteroids
        }).map(|(cluster_name, _)| cluster_name.clone()).unwrap_or("Unknown".to_string());

        println!("{:<25} {:<15.6} {:<15.2} {:<20} {:<15.6} {:<15}",
            asteroid.0, asteroid.2, asteroid.1 * 1_000_000.0, asteroid.3, asteroid.1, cluster); // prints the results
    }

    // establish an interactive lookup to manually search for asteroids by name
    use std::io::{self, Write}; 

    println!("\nEnter the name of an asteroid to retrieve details or type 'exit' to quit:"); // prompts the user to enter an asteroid name
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
            println!("{:<15}: {:.2} km/s", "Velocity", asteroid.1 * 1_000_000.0);
            println!("{:<15}: {}", "Date", asteroid.3);
            println!("{:<15}: {}", "Hazard Cluster", cluster);
        } else {
            println!("Asteroid '{}' not found in the dataset.", input);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv_reader::AsteroidData;

    fn mock_asteroid_data() -> Vec<AsteroidData> {
        vec![
            AsteroidData { 
                des: "2023 AB".to_string(),
                dist_min: 0.03, 
                v_rel: 15.0, 
                cd: "2023-01-01".to_string(),
            },
            AsteroidData { 
                des: "2023 XY".to_string(),
                dist_min: 0.1, 
                v_rel: 8.0, 
                cd: "2023-05-05".to_string(),
            },
            AsteroidData { 
                des: "2023 ZZ".to_string(),
                dist_min: 0.02, 
                v_rel: 20.0, 
                cd: "2023-07-07".to_string(),
            },
        ]
    }

    #[test]
    fn test_main_ranking_and_clustering() {
        let data = mock_asteroid_data();

        // test ranking
        let ranked_asteroids = rank_hazardous_asteroids(&data);
        assert_eq!(ranked_asteroids.len(), 3);
        assert!(ranked_asteroids[0].1 > ranked_asteroids[1].1, "Asteroids should be ranked by hazard score descending.");

        // test clustering
        let clusters = cluster_asteroids_by_hazard(&ranked_asteroids);
        assert!(clusters.contains_key("Highest Risk"));
        assert!(clusters.contains_key("Moderate Risk"));
        assert!(clusters["Highest Risk"].len() > 0, "At least one asteroid should be in the highest risk cluster.");
    }

    #[test]
    fn test_hazard_graph_building() {
        let data = mock_asteroid_data();
        let dist_threshold = 0.05;
        let velocity_threshold = 10.0;

        let hazard_graph = build_hazard_graph(&data, dist_threshold, velocity_threshold);

        // verify nodes
        assert_eq!(hazard_graph.node_count(), 2, "Graph should include only asteroids meeting the thresholds.");

        // verify edges
        assert!(hazard_graph.edge_count() > 0, "There should be at least one edge in the hazard graph.");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use csv_reader::AsteroidData;

    // helper function to generate mock asteroid data
    fn mock_asteroid_data() -> Vec<AsteroidData> {
        vec![
            AsteroidData { 
                des: "Asteroid A".to_string(),
                dist_min: 0.02, 
                v_rel: 20.0, 
                cd: "2024-01-01".to_string(),
            },
            AsteroidData { 
                des: "Asteroid B".to_string(),
                dist_min: 0.05, 
                v_rel: 10.0, 
                cd: "2024-02-01".to_string(),
            },
            AsteroidData { 
                des: "Asteroid C".to_string(),
                dist_min: 0.1, 
                v_rel: 5.0, 
                cd: "2024-03-01".to_string(),
            },
        ]
    }

    #[test]
    fn test_rank_hazardous_asteroids() {
        let data = mock_asteroid_data();

        // call the function
        let ranked = rank_hazardous_asteroids(&data);

        // verify the results
        assert_eq!(ranked.len(), 3, "There should be three asteroids ranked.");
        assert_eq!(ranked[0].0, "Asteroid A", "Asteroid A should be ranked first due to highest hazard score.");
        assert!(ranked[0].1 > ranked[1].1, "Hazard score of the first asteroid should be greater than the second.");
        assert!(ranked[1].1 > ranked[2].1, "Hazard score of the second asteroid should be greater than the third.");

        // verify the hazard score computation
        let asteroid_a_score = (data[0].v_rel / data[0].dist_min) / 1_000_000.0;
        assert!((ranked[0].1 - asteroid_a_score).abs() < 1e-6, "Hazard score of Asteroid A should match expected value.");
    }

    #[test]
    fn test_cluster_asteroids_by_hazard() {
        let data = mock_asteroid_data();
        let ranked = rank_hazardous_asteroids(&data);

        // call the function
        let clusters = cluster_asteroids_by_hazard(&ranked);

        // verify the results
        assert_eq!(clusters.len(), 4, "There should be four clusters.");

        // check the clustering of each asteroid
        for asteroid in ranked {
            let hazard_score = asteroid.1;
            let assigned_cluster = clusters.iter().find(|(_, asteroids)| {
                asteroids.iter().any(|(name, _, _, _)| name == &asteroid.0)
            });
            assert!(assigned_cluster.is_some(), "Each asteroid should belong to a cluster.");

            // verify correct cluster assignment based on hazard score
            if hazard_score < 0.01 {
                assert!(assigned_cluster.unwrap().0 == "Negligible Risk", "Asteroid with low hazard score should be in 'Negligible Risk'.");
            } else if hazard_score < 0.05 {
                assert!(assigned_cluster.unwrap().0 == "Low Risk", "Asteroid with moderate hazard score should be in 'Low Risk'.");
            } else if hazard_score < 0.1 {
                assert!(assigned_cluster.unwrap().0 == "Moderate Risk", "Asteroid with higher hazard score should be in 'Moderate Risk'.");
            } else {
                assert!(assigned_cluster.unwrap().0 == "Highest Risk", "Asteroid with the highest hazard score should be in 'Highest Risk'.");
            }
        }
    }
}

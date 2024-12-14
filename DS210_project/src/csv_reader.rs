use csv::ReaderBuilder;
use std::error::Error;

// Define a struct to represent the dataset row
#[derive(Debug)]
pub struct AsteroidData {
    pub des: String,
    pub orbit_id: String,
    pub jd: f64,
    pub cd: String,
    pub dist: f64,
    pub dist_min: f64,
    pub dist_max: f64,
    pub v_rel: f64,
    pub v_inf: f64,
    pub t_sigma_f: String,
}

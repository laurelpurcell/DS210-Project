// import 
use csv::ReaderBuilder;
use std::error::Error;

// define a struct to represent the rows of the asteroid dataset
#[derive(Debug)]
pub struct AsteroidData {
    // each type is a given data type based on their function in the dataset
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

// creates a function to parse the CSV file into a vector of AsteroidData
pub fn read_csv(file_path: String) -> Result<Vec<AsteroidData>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&file_path)?; // fixed variable name that calls the local path 

    let headers = reader.headers()?.clone();
    let des_idx = headers.iter().position(|h| h == "des").ok_or("Missing header 'des'")?;
    let orbit_id_idx = headers.iter().position(|h| h == "orbit_id").ok_or("Missing header 'orbit_id'")?;
    let jd_idx = headers.iter().position(|h| h == "jd").ok_or("Missing header 'jd'")?;
    let cd_idx = headers.iter().position(|h| h == "cd").ok_or("Missing header 'cd'")?;
    let dist_idx = headers.iter().position(|h| h == "dist").ok_or("Missing header 'dist'")?;
    let dist_min_idx = headers.iter().position(|h| h == "dist_min").ok_or("Missing header 'dist_min'")?;
    let dist_max_idx = headers.iter().position(|h| h == "dist_max").ok_or("Missing header 'dist_max'")?;
    let v_rel_idx = headers.iter().position(|h| h == "v_rel").ok_or("Missing header 'v_rel'")?;
    let v_inf_idx = headers.iter().position(|h| h == "v_inf").ok_or("Missing header 'v_inf'")?;
    let t_sigma_f_idx = headers.iter().position(|h| h == "t_sigma_f").ok_or("Missing header 't_sigma_f'")?;

    let mut records = Vec::new();

    // returns the results after the function is parsed; includes error case if there is missing data
    for result in reader.records() {
        let record = result?;
        records.push(AsteroidData {
            des: record.get(des_idx).ok_or("Missing value for 'des'")?.to_string(),
            orbit_id: record.get(orbit_id_idx).ok_or("Missing value for 'orbit_id'")?.to_string(),
            jd: record.get(jd_idx).ok_or("Missing value for 'jd'")?.parse()?,
            cd: record.get(cd_idx).ok_or("Missing value for 'cd'")?.to_string(),
            dist: record.get(dist_idx).ok_or("Missing value for 'dist'")?.parse()?,
            dist_min: record.get(dist_min_idx).ok_or("Missing value for 'dist_min'")?.parse()?,
            dist_max: record.get(dist_max_idx).ok_or("Missing value for 'dist_max'")?.parse()?,
            v_rel: record.get(v_rel_idx).ok_or("Missing value for 'v_rel'")?.parse()?,
            v_inf: record.get(v_inf_idx).ok_or("Missing value for 'v_inf'")?.parse()?,
            t_sigma_f: record.get(t_sigma_f_idx).ok_or("Missing value for 't_sigma_f'")?.to_string(),
        });
    }
    Ok(records)
}

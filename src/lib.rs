use pyo3::prelude::*;  
use std::collections::HashMap;  
use std::fs::File;  
use std::io::{Read, Write};  
use std::path::PathBuf;  
use serde::{Deserialize, Serialize};  
  
#[derive(Serialize, Deserialize)]  
struct Config {  
    paths: Vec<Vec<String>>,  
}  
  
#[pyfunction]  
fn do_job(config_file: &str) -> PyResult<()> {  
    // import config file  
    let mut file = File::open(config_file).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;  
    let config: Config = serde_json::from_reader(&mut file).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;  
  
    // summon config file for client  
    let mut cc: HashMap<String, Vec<(String, String)>> = HashMap::new();  
    for obj in config.paths {  
        let path = PathBuf::from(&obj[0]);  
        let file_stem = path.file_stem().and_then(|f| f.to_str());  
        if let Some(file_stem) = file_stem {  
            let files = vec![file_stem.to_string()]; // For now, we only handle one file per path.  
            if !cc.contains_key(&obj[2]) {  
                cc.insert(obj[2].clone(), Vec::new());  
            }  
            for file in files {  
                let filename = format!("{}/{}", obj[0], file);  
                let mut hash = String::new();  
                let mut file = File::open(&filename).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;  
                file.read_to_string(&mut hash).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;  
                cc.get_mut(&obj[2]).map(|val| val.push((filename, hash)));
            }  
        } else {  
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid path".to_string()));  
        }  
    }  
  
    // save to file  
    let mut file = File::create(config_file).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;  
    serde_json::to_writer(&mut file, &cc).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;  
    Ok(())  
}  
  
#[pymodule]  
fn serverConfigGenerator(_py: Python, m: &PyModule) -> PyResult<()> {  
    m.add_function(wrap_pyfunction!(do_job, m)?)?;  
    Ok(())  
}
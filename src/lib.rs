
pub mod app;
pub mod cluster;
pub mod math;

use std::fs::File;
use std::io::prelude::*;

pub fn read_csv(file_path: &str) -> Vec<Vec<String>> {
  let mut file = File::open(file_path).unwrap();
  let mut data = String::new();
  file.read_to_string(&mut data).unwrap();

  let mut csv = Vec::<Vec<String>>::new();
  let lines = data.split("\n");

  for line in lines {
    if line.trim().is_empty() {
      continue;
    }
    
    let columns: Vec<String> = line.split(",").map(|x| String::from(x)).collect();
    csv.push(columns.to_vec());
  }

  csv
}

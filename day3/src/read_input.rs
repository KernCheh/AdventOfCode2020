use std::fs::File;
use std::io::{self, BufRead};

/// Read an input file provided by the parameter `filename`.
/// Returns a line iterator.
pub fn read_input_file_as_lines(filename: &str) -> io::Lines<io::BufReader<std::fs::File>> {
  let f = File::open(filename).unwrap();
  return io::BufReader::new(f).lines();
}

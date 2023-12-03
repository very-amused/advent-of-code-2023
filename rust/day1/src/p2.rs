use std::{io::{BufReader, BufRead}, fs::File, error::Error, time::{Instant, Duration}};

const INPUT_FILE: &str = "input.txt"; // Use sample.txt for testing

// #region Structs
type Input = Vec<i32>; // Set input type, defaults to Vec<i32>

// #endregion

// Parse
fn parse() -> Result<Input, Box<dyn Error>> {
  // Open scanner to read input line by line
  let file = File::open(INPUT_FILE)?;
  let reader = BufReader::new(file);
  
  // Parse lines
  let mut input: Input = Vec::new();
  for l in reader.lines().map(|l| -> String {
    l.unwrap_or(String::new())
  }) {
    if l.len() == 0 {
			continue;
    }
    todo!();
  }
  Ok(input)
}

// Solve
fn solve(input: &mut Input) -> Result<String, Box<dyn Error>> {
  todo!()
}

pub fn run() -> (String, Duration) {
  // Parse
  let mut input = parse().expect("Failed to parse input");

  // Solve
  let start = Instant::now();
  let solution = solve(&mut input).expect("Failed to solve");
  let duration = start.elapsed();

  // Return solve time and solution
	(solution, duration)
}

use std::{io::{BufReader, BufRead}, fs::File, error::Error, time::{Instant, Duration}};
use crate::p1::*;

const INPUT_FILE: &str = "input.txt"; // Use sample.txt for testing

// #region Structs
type Input = Vec<Digits>;
// #endregion

const NUMBERS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

// Parse
fn parse() -> Result<Input, Box<dyn Error>> {
  // Open scanner to read input line by line
  let file = File::open(INPUT_FILE)?;
  let reader = BufReader::new(file);
  
  // Parse lines
  let mut input: Input = Vec::new();


	// Match vectors for each line
  for l in reader.lines().map(|l| -> String {
    l.unwrap_or(String::new())
  }) {
    if l.len() == 0 {
			continue;
    }
		let mut left: usize = 0;
		let mut left_index: usize = usize::MAX;
		let mut right: usize = 0;
		let mut right_index: usize = 0;

		// Find digit substring matches
		for (n, name) in NUMBERS.iter().enumerate() {
			// Left digit match
			let lmatch = [l.find(n.to_string().as_str()), l.find(name)].iter()
				.filter_map(|i| i.as_ref()) // Filter out None values where the number doesn't have a match
				.map(|i| *i)
				.min();
			// Right digit match
			let rmatch = [l.rfind(n.to_string().as_str()), l.rfind(name)].iter()
				.filter_map(|i| i.as_ref())
				.map(|i| *i)
				.max();

			// Update the leftmost/rightmost digits found in the line
			match lmatch {
				Some(i) if i < left_index => {
					left = n;
					left_index = i;
				},
				_ => {}
			}
			match rmatch {
				Some(i) if i >= right_index => {
					right = n;
					right_index = i;
				},
				_ => {}
			}
		}
		input.push(Digits(left as i32, right as i32));
  }
  Ok(input)
}

// Solve
fn solve(input: &mut Input) -> Result<String, Box<dyn Error>> {
	let mut sum = 0;
	for pair in input {
		sum += pair.calibration_value();
	}
	Ok(sum.to_string())
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

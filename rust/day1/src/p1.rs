use std::{io::{BufReader, BufRead}, fs::File, error::Error, time::{Instant, Duration}};

const INPUT_FILE: &str = "input.txt"; // Use sample.txt for testing

// #region Structs
pub struct Digits (pub i32, pub i32);

impl Digits {
	pub fn calibration_value(&self) -> i32 {
		(10 * self.0) + self.1
	}
}

type Input = Vec<Digits>;
// #endregion

// Parse
fn parse() -> Result<Input, Box<dyn Error>> {
  // Open scanner to read input line by line
  let file = File::open(INPUT_FILE)?;
  let reader = BufReader::new(file);
  
  // Parse lines
  let mut input = Vec::new();
  for l in reader.lines().map(|l| -> String {
    l.unwrap_or(String::new())
  }) {
    if l.len() == 0 {
			continue;
    }
		// Find first/last single digit characters
		let mut digits = Digits (-1, -1);
		for c in l.split("") {
			// Ignore non-digit characters
			if let Ok(num) = c.parse() {
				if digits.0 == -1 {
					digits.0 = num;
				}
				digits.1 = num;
			}
		}
		input.push(digits);
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

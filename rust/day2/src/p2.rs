use std::{io::{BufReader, BufRead}, fs::File, fmt::Display, error::Error, time::Instant, str::FromStr, num::ParseIntError};
use super::p1::Game;

const INPUT_FILE: &str = "input.txt"; // Use sample.txt for testing


// #region Structs
type Input = Vec<Game>; // Set input type, defaults to Vec<i32>
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
		input.push(l.parse()?);
  }
  Ok(input)
}

// Solve
fn solve(input: &mut Input) -> Result<String, Box<dyn Error>> {

	// Sum of the power value for each game
	let mut sum: u32 = 0;

	for game in input {
		// Necessary cube counts for the game to be possible (max count of that color shown in a round)
		let mut red: u32 = 0;
		let mut green: u32 = 0;
		let mut blue: u32 = 0;

		for round in &game.rounds {
			if round.red > red {
				red = round.red;
			}
			if round.green > green {
				green = round.green;
			}
			if round.blue > blue {
				blue = round.blue;
			}
		}
		let power = red * green * blue;
		sum += power;
	}

	Ok(sum.to_string())

}

pub fn part2() {
  // Parse
  let mut input = parse().expect("Failed to parse input");

  // Solve
  let start = Instant::now();
  let solution = solve(&mut input).expect("Failed to solve");

  // Report solve time and solution
  let duration = start.elapsed();
  println!("Solved in \x1b[34m{:?}\x1b[0m", duration);
  println!("Solution: \x1b[32m{}\x1b[0m", solution);
}

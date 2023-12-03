use std::{io::{BufReader, BufRead}, fs::File, fmt::Display, error::Error, time::Instant, str::FromStr, num::ParseIntError};

const INPUT_FILE: &str = "input.txt"; // Use sample.txt for testing

// #region Parse error
#[derive(Debug)]
pub enum ParseError {
  InvalidInput(String),
	ParseIntError(ParseIntError)
}

impl Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidInput(s) => write!(f, "{}", s),
			Self::ParseIntError(err) => err.fmt(f)
    }
  }
}

impl Error for ParseError {}

macro_rules! parse_err {
  ($($arg:tt)*) => {
    Err(ParseError::InvalidInput(format!($($arg)*)))
  };
}
#[allow(unused_macros)]
macro_rules! box_parse_err {
  ($($arg:tt)*) => {
    Err(Box::new(ParseError::InvalidInput(format!($($arg)*))))
  };
}

// #endregion

// #region Structs
pub struct Round {
	pub red: u32,
	pub green: u32,
	pub blue: u32
}

impl FromStr for Round {
	type Err = ParseError;

	// Parse round record e.g "1 blue, 2 green, 3 red"
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut round = Self {
			red: 0,
			green: 0,
			blue: 0
		};
		for record in s.split(", ") {
			let (count, color) = {
				let parts: Vec<&str> = record.splitn(2, " ").collect();
				(
					parts[0].parse::<u32>().map_err(ParseError::ParseIntError)?,
					parts[1])
			};
			match color {
				"red" => round.red = count,
				"green" => round.green = count,
				"blue" => round.blue = count,
				_ => {}
			}
		}

		Ok(round)
	}
}

pub struct Game {
	pub id: u32,
	pub rounds: Vec<Round>
}

impl FromStr for Game {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (header, rounds) = { 
			let parts: Vec<&str> = s.splitn(2, ": ").collect(); 
			(parts[0], parts[1])
		};

		// Create game struct
		let mut game = Self {
			id: 0,
			rounds: vec![]
		};

		// Parse game ID from header
		game.id = header.splitn(2, " ")
			.last().unwrap_or("")
			.parse().map_err(|_| ParseError::InvalidInput("Something".to_string()))?;

		// Parse game rounds
		game.rounds = rounds.split("; ")
			.map(|round| {
				round.parse().map_err(|_| {
					ParseError::InvalidInput(format!("Failed to parse round in game {}", game.id))
				})
			}).collect::<Result<Vec<Round>, ParseError>>()?;

		Ok(game)
	}
}

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
	// Max counts for given cube colors, determining whether a game is possible
	const RED_MAX: u32 = 12;
	const GREEN_MAX: u32 = 13;
	const BLUE_MAX: u32 = 14;

	// Sum of the IDs of all possible games
	let mut sum: u32 = 0;

	for game in input {
		let mut possible = true;
		for round in &game.rounds {
			if round.red > RED_MAX || round.green > GREEN_MAX || round.blue > BLUE_MAX {
				possible = false;
				break;
			}
		}
		if possible {
			sum += game.id;
		}
	}

	Ok(sum.to_string())

}

pub fn part1() {
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

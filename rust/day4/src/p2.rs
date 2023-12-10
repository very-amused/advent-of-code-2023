use std::{io::{BufReader, BufRead}, fs::File, error::Error, time::{Instant, Duration}, str::FromStr};

const INPUT_FILE: &str = "input.txt"; // Use sample.txt for testing

// #region Structs
#[derive(Clone)]
struct Card {
	winning_numbers: Vec<u32>,
	numbers: Vec<u32>
}
type Input = Vec<Card>;

impl Card {
	fn sort_winning_numbers(&mut self) {
		self.winning_numbers.sort_unstable();
	}

	// self.winning_numbers must be sorted before score can be calculated
	pub fn score(&self, cards: &Vec<Card>, index: usize) -> u32 {
		// Calculate number of cards won
		let mut cards_won_count = 0;
		for n in &self.numbers {
			if self.winning_numbers.binary_search(n).is_ok() {
				cards_won_count += 1;
			}
		}

		// Calculate score
		let mut score: u32 = 1;
		let cards_won_range = index+1..=index+cards_won_count;
		for i in cards_won_range {
			score += cards[i].score(cards, i);
		}
		score
	}
}

impl FromStr for Card {
	type Err = crate::error::ParseError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// Split things
		let (_, body) = s.split_once(": ").expect("failed to split body");
		let (winning_list, num_list) = body.split_once(" | ").expect("failed to split number lists");
		let parse_list = |s: &str| -> Vec<u32> {
			s.split(" ")
				.filter_map(|n| n.parse::<u32>().ok())
				.collect()
		};

		// Parse number lists
		let mut card = Self {
			winning_numbers: parse_list(winning_list),
			numbers: parse_list(num_list)
		};
		card.sort_winning_numbers();

		Ok(card)
	}
}
// #endregion

// Parse
fn parse() -> Result<Input, Box<dyn Error>> {
  // Open scanner to read input line by line
  let file = File::open(INPUT_FILE)?;
  let reader = BufReader::new(file);
  
  // Parse lines
  let mut input: Input = Vec::new();
  for l in reader.lines().filter_map(|l| l.ok()) {
    if l.len() == 0 {
			continue;
    }
		if let Ok(card) = l.parse::<Card>() {
			input.push(card);
		}
  }
  Ok(input)
}

// Solve
fn solve(input: &mut Input) -> Result<String, Box<dyn Error>> {
	Ok(
		input.iter().enumerate()
		.map(|(i, card)| card.score(&input, i))
		.sum::<u32>().to_string())
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

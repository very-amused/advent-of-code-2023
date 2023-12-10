use std::{io::{BufReader, BufRead}, fs::File, error::Error, time::{Instant, Duration}};

const INPUT_FILE: &str = "input.txt"; // Use sample.txt for testing

// #region Structs
struct Number {
	value: u32,
	x: usize,
	x_end: usize,
	y: usize
}

struct EngineSchematic {
	grid: Vec<Vec<bool>>, // True in cells that hold symbols
	numbers: Vec<Number>
}

type Input = EngineSchematic;
// #endregion

// Parse
fn parse() -> Result<Input, Box<dyn Error>> {
  // Open scanner to read input line by line
  let file = File::open(INPUT_FILE)?;
  let reader = BufReader::new(file);
  
  // Parse lines
  let mut input: Input = EngineSchematic {
		grid: Vec::new(),
		numbers: Vec::new()
	};
  for (y, l) in reader.lines().filter_map(|l| l.ok()).enumerate() {
    if l.len() == 0 {
			continue;
    }
		let mut grid_row: Vec<bool> = Vec::with_capacity(l.len());
		grid_row.resize(l.len(), false);

		let chars: Vec<char> = l.chars().collect();
		for (x, c) in chars.iter().enumerate() {
			// Parse numbers
			if c.is_ascii_digit() {
				// Only parse numbers from start
				if !(x == 0 || !chars[x-1].is_ascii_digit()) {
					continue;
				}

				let mut end = x+1; // Exclusive end index
				while end < chars.len() && chars[end].is_ascii_digit() {
					end += 1;
				}

				let num_str: String = chars[x..end].iter().collect();
				if let Ok(num) = num_str.parse::<u32>() {
					input.numbers.push(Number {
						value: num,
						x,
						x_end: end,
						y
					});
				}
				continue;
			}
			grid_row[x] = *c != '.'; // Set symbol cells
		}
		input.grid.push(grid_row);
  }
  Ok(input)
}

// Solve
fn solve(input: &mut Input) -> Result<String, Box<dyn Error>> {
	let mut sum: u32 = 0;
	let row_len = input.grid[0].len();
	for n in &input.numbers {
		// Calculate search ranges
		let x = if n.x == 0 { n.x } else { n.x-1 };
		let x_end = if n.x_end == row_len { n.x_end } else { n.x_end+1 };
		let y = if n.y == 0 { n.y } else { n.y-1 };
		let y_end = if n.y == input.grid.len()-1 { n.y+1 } else { n.y+2 };

		// Check if the number has an adjacent symbol
		let mut valid = false;
		for row in &input.grid[y..y_end] {
			for cell in &row[x..x_end] {
				if *cell {
					valid = true;
					break;
				}
			}
			if valid {
				break;
			}
		}

		if valid {
			sum += n.value;
		}
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

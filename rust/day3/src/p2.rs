use std::{io::{BufReader, BufRead}, fs::File, error::Error, time::{Instant, Duration}};

const INPUT_FILE: &str = "input.txt"; // Use sample.txt for testing

// #region Structs
struct Number {
	value: u32,
	x: usize,
	x_end: usize,
	y: usize
}

#[derive(Clone)]
struct Gear {
	numbers: Vec<u32>
}

impl Gear {
	// Get a gear's ratio value
	fn ratio(&self) -> u32 {
		if self.numbers.len() == 2 {
			self.numbers[0] * self.numbers[1]
		} else {
			0
		}
	}
}

struct EngineSchematic {
	grid: Vec<Vec<Option<Gear>>>,
	numbers: Vec<Number>,
	gears: Vec<(usize, usize)> // Gear indexes
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
		numbers: Vec::new(),
		gears: Vec::new()
	};
  for (y, l) in reader.lines().filter_map(|l| l.ok()).enumerate() {
    if l.len() == 0 {
			continue;
    }
		let mut grid_row: Vec<Option<Gear>> = Vec::with_capacity(l.len());
		grid_row.resize(l.len(), None);

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
			if *c == '*' {
				let gear = Gear {
					numbers: Vec::new()
				};
				grid_row[x] = Some(gear);
				input.gears.push((y, x));
			}
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

		// Add the number to all adjacent gears
		for row in &mut input.grid[y..y_end] {
			for cell in &mut row[x..x_end] {
				if let Some(gear) = cell {
					gear.numbers.push(n.value);
				}
			}
		}
	}

	// Sum the ratio of all gears
	for (y, x) in &input.gears {
		if let Some(gear) = &input.grid[*y][*x] {
			sum += gear.ratio();
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

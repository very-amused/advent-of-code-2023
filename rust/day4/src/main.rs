use std::time::Duration;

mod p1;
mod p2;
#[macro_use]
mod error;

fn main() {
	let mut solution: String;
	let mut duration: Duration;
	macro_rules! report_solution {
		() => {
			println!("Solved in \x1b[34m{:?}\x1b[0m", duration);
			println!("Solution: \x1b[32m{}\x1b[0m", solution);
		}
	}

	println!("Part 1:");
	(solution, duration) = p1::run();
	report_solution!();

	println!("Part 2:");
	(solution, duration) = p2::run();
	report_solution!();
}


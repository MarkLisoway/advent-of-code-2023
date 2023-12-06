fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> i64 {
	parse(input)
		.iter()
		.map(|race| race.compute_number_of_ways_to_win())
		.reduce(|acc, wins| acc * wins)
		.expect("At least two races to exist")
}

fn part_2(input: &str) -> i64 {
	parse_single_race(input).compute_number_of_ways_to_win()
}

fn parse(input: &str) -> Vec<Race> {
	let numbers: Vec<Vec<i64>> = input
		.lines()
		.map(|line| {
			let (_, numbers) = line.split_once(':').expect("Split character to exist");
			numbers
				.split_ascii_whitespace()
				.map(|n| n.parse::<i64>().expect("Number to parse"))
				.collect()
		})
		.collect();

	numbers[0]
		.iter()
		.zip(numbers[1].iter())
		.map(|(time, distance)| Race {
			time: *time,
			distance: *distance,
		})
		.collect()
}

fn parse_single_race(input: &str) -> Race {
	let numbers: Vec<i64> = input
		.lines()
		.map(|line| {
			let (_, numbers) = line.split_once(':').expect("Split character to exist");
			let numbers: Vec<_> = numbers.split_ascii_whitespace().collect();
			let numbers = numbers.join("");
			numbers.parse::<i64>().expect("Number to parse")
		})
		.collect();

	Race {
		time: numbers[0],
		distance: numbers[1],
	}
}

#[derive(Debug)]
struct Race {
	pub time: i64,
	pub distance: i64,
}

impl Race {
	fn compute_number_of_ways_to_win(&self) -> i64 {
		(0..=self.time)
			.map(|hold_time| {
				let time_left = self.time - hold_time;
				let distance = time_left * hold_time;

				match distance > self.distance {
					true => 1,
					false => 0,
				}
			})
			.sum::<i64>()
	}
}

#[cfg(test)]
mod tests_day_01 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
Time:      7  15   30
Distance:  9  40  200
"#
		.trim();

		assert_eq!(part_1(input), 288);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
Time:      7  15   30
Distance:  9  40  200
"#
		.trim();

		assert_eq!(part_2(input), 71503);
	}
}

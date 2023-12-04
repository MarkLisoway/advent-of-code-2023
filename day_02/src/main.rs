fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u32 {
	let max_red = 12;
	let max_green = 13;
	let max_blue = 14;

	input
		.lines()
		.map(|line| {
			let game_num = extract_game_number(line);

			let game_valid = line
				.split_once(':')
				.expect("Game rounds separator should exist")
				.1
				.split(';')
				.map(|round| {
					let pulls = round.split(',');

					let mut all_pulls_valid = true;

					for pull in pulls {
						let (num, colour) = get_pull_num_and_colour(pull);

						let valid = (colour.starts_with('r') && num <= max_red)
							|| (colour.starts_with('g') && num <= max_green)
							|| (colour.starts_with('b') && num <= max_blue);

						if !valid {
							all_pulls_valid = false;
							break;
						}
					}

					all_pulls_valid
				})
				.all(|valid| valid);

			match game_valid {
				true => game_num,
				false => 0,
			}
		})
		.sum()
}

fn part_2(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let (r_max, g_max, b_max) = line
				.split_once(':')
				.expect("Game rounds separator should exist")
				.1
				.split(';')
				.map(|round| {
					let mut red = 0;
					let mut green = 0;
					let mut blue = 0;

					let pulls = round.split(',');
					for pull in pulls {
						let (num, colour) = get_pull_num_and_colour(pull);

						match colour {
							"red" => red = red.max(num),
							"green" => green = green.max(num),
							"blue" => blue = blue.max(num),
							_ => unreachable!("Other colours do not exist"),
						}
					}

					(red, green, blue)
				})
				.reduce(|l, r| (l.0.max(r.0), l.1.max(r.1), l.2.max(r.2)))
				.expect("Iterator will not be empty");

			r_max * g_max * b_max
		})
		.sum()
}

fn extract_game_number(line: &str) -> u32 {
	let game = line
		.split_once(':')
		.expect("Game numer separator should exist")
		.0;
	let num = game
		.split_ascii_whitespace()
		.last()
		.expect("Game number to be last");
	num.parse().expect("Number to parse successfully")
}

fn get_pull_num_and_colour(pull: &str) -> (u32, &str) {
	let mut pull_split = pull.split_ascii_whitespace();
	let num = pull_split.next().expect("Pull number should exist");
	let num: u32 = num.parse().expect("Number to parse");

	let colour = pull_split.next().expect("Colour code to exist");

	(num, colour)
}

#[cfg(test)]
mod tests_day_02 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#
		.trim();

		assert_eq!(part_1(input), 8);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#
		.trim();

		assert_eq!(part_2(input), 2286);
	}
}

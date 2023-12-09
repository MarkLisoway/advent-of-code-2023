fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(_input: &str) -> u64 {
	1
}

fn part_2(_input: &str) -> u64 {
	1
}

#[cfg(test)]
mod tests_day_00 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"

"#
		.trim();

		assert_eq!(part_1(input), 1);
	}

	#[test]
	fn part_02_example() {
		let input = r#"

"#
		.trim();

		assert_eq!(part_2(input), 1);
	}
}

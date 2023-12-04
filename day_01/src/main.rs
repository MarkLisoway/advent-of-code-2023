fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let numbers: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();

			let first = *numbers.first().expect("Expect one number to always exist");
			let last = *numbers.last().expect("Expect one number to always exist");

			(10 * first) + last
		})
		.sum()
}

fn part_2(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let mut digits = vec![];

			for (i, char) in line.chars().enumerate() {
				let digit = match char.to_digit(10) {
					Some(d) => Some(d),
					None => {
						let sub_line = &line[i..];

						if sub_line.starts_with("one") {
							Some(1)
						} else if sub_line.starts_with("two") {
							Some(2)
						} else if sub_line.starts_with("three") {
							Some(3)
						} else if sub_line.starts_with("four") {
							Some(4)
						} else if sub_line.starts_with("five") {
							Some(5)
						} else if sub_line.starts_with("six") {
							Some(6)
						} else if sub_line.starts_with("seven") {
							Some(7)
						} else if sub_line.starts_with("eight") {
							Some(8)
						} else if sub_line.starts_with("nine") {
							Some(9)
						} else {
							None
						}
					}
				};

				if let Some(digit) = digit {
					digits.push(digit);
				}
			}

			let first = *digits.first().expect("Expect one number to always exist");
			let last = *digits.last().expect("Expect one number to always exist");

			(10 * first) + last
		})
		.sum()
}

#[cfg(test)]
mod tests_day_01 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#
		.trim();

		assert_eq!(part_1(input), 142);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#
		.trim();

		assert_eq!(part_2(input), 281);
	}
}

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> i64 {
	input
		.lines()
		.map(|line| {
			let mut results = vec![];

			let row_1 = line
				.split_ascii_whitespace()
				.map(|n| n.parse::<i64>().expect("Number to parse"))
				.collect::<Vec<_>>();

			results.push(row_1);

			loop {
				let mut row = vec![];
				let last_row = results.last().expect("Last row to exist");

				for window in last_row.windows(2) {
					let current = window[0];
					let next = window[1];
					row.push(next - current);
				}

				let all_zeros = row.iter().all(|v| *v == 0);

				results.push(row);

				if all_zeros {
					break;
				}
			}

			let mut added_val = 0;
			for row in results.iter().rev() {
				added_val += row.last().expect("Last value to exist");
			}

			added_val
		})
		.sum()
}

fn part_2(input: &str) -> i64 {
	input
		.lines()
		.map(|line| {
			let mut results = vec![];

			let row_1 = line
				.split_ascii_whitespace()
				.map(|n| n.parse::<i64>().expect("Number to parse"))
				.collect::<Vec<_>>();

			results.push(row_1);

			loop {
				let mut row = vec![];
				let last_row = results.last().expect("Last row to exist");

				for window in last_row.windows(2) {
					let current = window[0];
					let next = window[1];
					row.push(next - current);
				}

				let all_zeros = row.iter().all(|v| *v == 0);

				results.push(row);

				if all_zeros {
					break;
				}
			}

			results.iter_mut().for_each(|r| r.reverse());

			let mut added_val = 0;
			for row in results.iter().rev() {
				added_val = row.last().expect("Last value to exist") - added_val;
				dbg!(added_val);
			}

			added_val
		})
		.sum()
}

#[cfg(test)]
mod tests_day_09 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#
		.trim();

		assert_eq!(part_1(input), 114);
	}

	#[test]
	fn part_01_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_1(input), 2098530125);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#
		.trim();

		assert_eq!(part_2(input), 2);
	}

	#[test]
	fn part_02_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_2(input), 1016);
	}
}

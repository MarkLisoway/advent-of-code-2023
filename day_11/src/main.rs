use std::collections::HashSet;
use std::time::Instant;

fn main() {
	let input = include_str!("part_1_input.txt");

	let input = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#
	.trim();

	let now = Instant::now();
	dbg!(part_1(input));
	dbg!(part_2(input));
	dbg!(Instant::now() - now);
}

fn part_1(input: &str) -> u64 {
	let rows = parse(input);

	let galaxies: Vec<_> = rows
		.iter()
		.enumerate()
		.flat_map(|(y, row)| {
			row.chars()
				.enumerate()
				.filter_map(|(x, c)| match c == '#' {
					true => Some(Point { x, y }),
					false => None,
				})
				.collect::<Vec<_>>()
		})
		.collect();

	let mut total = 0;
	for i in 0..galaxies.len() {
		for j in (i + 1)..galaxies.len() {
			let current = &galaxies[i];
			let next = &galaxies[j];

			let x_diff = current.x.abs_diff(next.x);
			let y_diff = current.y.abs_diff(next.y);

			total += x_diff + y_diff;
		}
	}

	dbg!(total);

	1
}

fn part_2(input: &str) -> u64 {
	let rows = parse_2(input);

	let mut empty_rows = HashSet::new();
	let mut empty_cols = HashSet::new();

	for y in 0..rows.len() {
		if rows[y].chars().all(|c| c == '.') {
			empty_rows.insert(y);
		}
	}

	for x in 0..rows.first().unwrap().len() {
		let mut empty = true;

		for y in 0..rows.len() {
			if rows[y].chars().nth(x).unwrap() != '.' {
				empty = false;
			}
		}

		if empty {
			empty_cols.insert(x);
		}
	}

	let galaxies: Vec<_> = rows
		.iter()
		.enumerate()
		.flat_map(|(y, row)| {
			row.chars()
				.enumerate()
				.filter_map(|(x, c)| match c == '#' {
					true => Some(Point { x, y }),
					false => None,
				})
				.collect::<Vec<_>>()
		})
		.collect();

	let mut total = 0;
	for i in 0..galaxies.len() {
		for j in (i + 1)..galaxies.len() {
			let current = &galaxies[i];
			let next = &galaxies[j];

			let min_x = current.x.min(next.x);
			let max_x = current.x.max(next.x);

			let min_y = current.y.min(next.y);
			let max_y = current.y.max(next.y);

			let x_crosses = (min_x..=max_x)
				.map(|x| match empty_cols.contains(&x) {
					true => 1,
					false => 0,
				})
				.sum::<i64>();

			let y_crosses = (min_y..=max_y)
				.map(|y| match empty_rows.contains(&y) {
					true => 1,
					false => 0,
				})
				.sum::<i64>();

			let x_diff =
				(current.x as i64 - next.x as i64).abs() - x_crosses + (x_crosses * 1_000_000);
			let y_diff =
				(current.y as i64 - next.y as i64).abs() - y_crosses + (y_crosses * 1_000_000);

			total += x_diff + y_diff;
		}
	}

	dbg!(total);

	1
}

fn parse(input: &str) -> Vec<String> {
	let mut rows = vec![];

	input.lines().for_each(|line| {
		if line.chars().all(|c| c == '.') {
			rows.push(line.to_string());
		}

		rows.push(line.to_string());
	});

	let mut empty_cols = vec![];
	for x in 0..rows.first().unwrap().len() {
		let mut is_empty = true;

		for y in 0..rows.len() {
			if rows[y].chars().nth(x).unwrap() != '.' {
				is_empty = false;
			}
		}

		if is_empty {
			empty_cols.push(x);
		}
	}

	let mut added = 0;
	for x in empty_cols {
		for y in 0..rows.len() {
			let row = rows.get_mut(y).unwrap();
			row.insert(x + added, '.');
		}

		added += 1;
	}

	rows
}

fn parse_2(input: &str) -> Vec<String> {
	let mut rows = vec![];

	input.lines().for_each(|line| {
		rows.push(line.to_string());
	});

	// let mut empty_cols = vec![];
	// for x in 0..rows.first().unwrap().len() {
	// 	let mut is_empty = true;
	//
	// 	for y in 0..rows.len() {
	// 		if rows[y].chars().nth(x).unwrap() != '.' {
	// 			is_empty = false;
	// 		}
	// 	}
	//
	// 	if is_empty {
	// 		empty_cols.push(x);
	// 	}
	// }
	//
	// let mut added = 0;
	// for x in empty_cols {
	// 	for y in 0..rows.len() {
	// 		let row = rows.get_mut(y).unwrap();
	// 		row.insert(x + added, '.');
	// 	}
	//
	// 	added += 1;
	// }

	rows
}

#[derive(Debug)]
struct Point {
	pub x: usize,
	pub y: usize,
}

#[cfg(test)]
mod tests_day_11 {
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

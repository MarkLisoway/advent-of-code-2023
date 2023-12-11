use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let universe = parse(input);
	universe.travel_galaxies(2)
}

fn part_2(input: &str) -> u64 {
	let universe = parse(input);
	universe.travel_galaxies(1_000_000)
}

fn parse(input: &str) -> Universe {
	let rows = input.lines().collect::<Vec<_>>();

	let galaxies = rows
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

	let empty_rows = rows
		.iter()
		.enumerate()
		.filter_map(|(y, row)| match row.chars().all(|c| c == '.') {
			true => Some(y),
			false => None,
		})
		.collect::<HashSet<_>>();

	let row_len = rows.first().expect("At least one row to exist").len();
	let empty_cols = (0..row_len)
		.filter(|x| {
			rows.iter()
				.map(|row| row.chars().nth(*x).expect("Column character to exist"))
				.all(|c| c == '.')
		})
		.collect::<HashSet<_>>();

	Universe {
		galaxies,
		empty_rows,
		empty_cols,
	}
}

#[derive(Debug)]
struct Point {
	pub x: usize,
	pub y: usize,
}

#[derive(Debug)]
struct Universe {
	pub galaxies: Vec<Point>,
	pub empty_rows: HashSet<usize>,
	pub empty_cols: HashSet<usize>,
}

impl Universe {
	fn travel_galaxies(&self, scaling: u64) -> u64 {
		let combinations = self
			.galaxies
			.iter()
			.tuple_combinations()
			.collect::<Vec<_>>();

		combinations
			.par_iter()
			.map(|(l, r)| {
				let x_distance = compute_distance(l.x, r.x, &self.empty_cols, scaling);
				let y_distance = compute_distance(l.y, r.y, &self.empty_rows, scaling);

				x_distance + y_distance
			})
			.sum()
	}
}

#[inline]
fn compute_distance(l: usize, r: usize, spaces: &HashSet<usize>, scaling: u64) -> u64 {
	let crosses = compute_crosses(l, r, spaces);

	l.abs_diff(r) as u64 + (crosses * scaling) - crosses
}

#[inline]
fn compute_crosses(l: usize, r: usize, spaces: &HashSet<usize>) -> u64 {
	let min = l.min(r);
	let max = l.max(r);
	let traverses = (min..=max).collect();

	spaces.intersection(&traverses).count() as u64
}

#[cfg(test)]
mod tests_day_11 {
	use super::*;

	#[test]
	fn part_01_example() {
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

		assert_eq!(part_1(input), 374);
	}

	#[test]
	fn part_02_test() {
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

		assert_eq!(part_2(input), 82000210);
	}
}

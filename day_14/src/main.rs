use std::collections::HashMap;

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let mut lines = parse(input);

	tilt_north(&mut lines);

	compute_weight(&lines)
}

fn part_2(input: &str) -> u64 {
	let mut lines = parse(input);

	let mut memo = HashMap::new();
	let mut memo_by_index = HashMap::new();
	let mut memo_index = 0;

	// Loop until we find a pattern we've already seen
	// which already points to another pattern.
	// This means we have found the start of a loop.
	let (start, end) = loop {
		if let Some((_, start)) = memo.get(&lines) {
			break (*start, memo_index);
		}

		let before = lines.clone();

		tilt_north(&mut lines);
		tilt_west(&mut lines);
		tilt_south(&mut lines);
		tilt_east(&mut lines);

		memo.insert(before, (lines.clone(), memo_index));
		memo_by_index.insert(memo_index, lines.clone());
		memo_index += 1;
	};

	let loop_length = end - start;
	let offset = 1_000_000_000 - start;
	let loop_index = offset % loop_length;
	let final_index = start + loop_index - 1;

	let result = memo_by_index
		.get(&final_index)
		.expect("Result pattern to exist");

	compute_weight(result)
}

fn parse(input: &str) -> Vec<Vec<Rock>> {
	input
		.lines()
		.map(|line| {
			line.chars()
				.map(|c| match c {
					'O' => Rock::Round,
					'#' => Rock::Cube,
					'.' => Rock::Empty,
					_ => unreachable!(),
				})
				.collect::<Vec<_>>()
		})
		.collect()
}

fn compute_weight(grid: &[Vec<Rock>]) -> u64 {
	grid.iter()
		.enumerate()
		.map(|(y, line)| {
			let height = grid.len() - y;
			let count = line.iter().filter(|r| **r == Rock::Round).count();

			(height * count) as u64
		})
		.sum::<u64>()
}

fn tilt_north(grid: &mut [Vec<Rock>]) {
	let len = grid[0].len();

	for y in 0..grid.len() {
		for x in 0..len {
			tilt_tile_north(grid, x, y);
		}
	}
}

#[inline]
fn tilt_tile_north(grid: &mut [Vec<Rock>], x: usize, y: usize) {
	if matches!(grid[y][x], Rock::Cube | Rock::Round) {
		return;
	}

	for n_y in (y + 1)..grid.len() {
		let next = grid[n_y][x];

		if matches!(next, Rock::Cube) {
			break;
		}

		if matches!(next, Rock::Round) {
			grid[y][x] = Rock::Round;
			grid[n_y][x] = Rock::Empty;
			break;
		}
	}
}

fn tilt_west(grid: &mut [Vec<Rock>]) {
	let len = grid[0].len();
	for y in 0..grid.len() {
		for x in 0..len {
			tilt_tile_west(grid, x, y);
		}
	}
}

#[inline]
fn tilt_tile_west(grid: &mut [Vec<Rock>], x: usize, y: usize) {
	if matches!(grid[y][x], Rock::Cube | Rock::Round) {
		return;
	}

	for n_x in (x + 1)..grid[0].len() {
		let next = grid[y][n_x];

		if matches!(next, Rock::Cube) {
			break;
		}

		if matches!(next, Rock::Round) {
			grid[y][x] = Rock::Round;
			grid[y][n_x] = Rock::Empty;
			break;
		}
	}
}

fn tilt_south(grid: &mut [Vec<Rock>]) {
	let len = grid[0].len();
	for y in (0..grid.len()).rev() {
		for x in 0..len {
			tilt_tile_south(grid, x, y);
		}
	}
}

#[inline]
fn tilt_tile_south(grid: &mut [Vec<Rock>], x: usize, y: usize) {
	if matches!(grid[y][x], Rock::Cube | Rock::Round) {
		return;
	}

	for n_y in (0..y).rev() {
		let next = grid[n_y][x];

		if matches!(next, Rock::Cube) {
			break;
		}

		if matches!(next, Rock::Round) {
			grid[y][x] = Rock::Round;
			grid[n_y][x] = Rock::Empty;
			break;
		}
	}
}

fn tilt_east(grid: &mut [Vec<Rock>]) {
	let len = grid[0].len();
	for y in 0..grid.len() {
		for x in (0..len).rev() {
			tilt_tile_east(grid, x, y);
		}
	}
}

#[inline]
fn tilt_tile_east(grid: &mut [Vec<Rock>], x: usize, y: usize) {
	if matches!(grid[y][x], Rock::Cube | Rock::Round) {
		return;
	}

	for n_x in (0..x).rev() {
		let next = grid[y][n_x];

		if matches!(next, Rock::Cube) {
			break;
		}

		if matches!(next, Rock::Round) {
			grid[y][x] = Rock::Round;
			grid[y][n_x] = Rock::Empty;
			break;
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Rock {
	Round,
	Cube,
	Empty,
}

#[cfg(test)]
mod tests_day_14 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#
		.trim();

		assert_eq!(part_1(input), 136);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#
		.trim();

		assert_eq!(part_2(input), 64);
	}
}

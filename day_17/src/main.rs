use pathfinding::matrix::{directions, Matrix};
use pathfinding::prelude::astar;

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let grid = parse(input);

	find_path::<1, 3>(&grid)
}

fn part_2(input: &str) -> u64 {
	let grid = parse(input);

	find_path::<4, 10>(&grid)
}

fn parse(input: &str) -> Matrix<u64> {
	input
		.lines()
		.map(|line| {
			line.chars()
				.map(|c| c.to_digit(10).expect("Digit to parse") as u64)
		})
		.collect::<Matrix<u64>>()
}

fn find_path<const MIN: u64, const MAX: u64>(grid: &Matrix<u64>) -> u64 {
	let start = State {
		position: (0, 0),
		direction: (0, 0),
		distance: 0,
	};

	let end = (grid.rows - 1, grid.columns - 1);

	let path = astar(
		&start,
		|state| match state.distance >= MIN || (state.direction.0 == 0 && state.direction.1 == 0) {
			true => compute_neighbouring_successors::<MAX>(state, grid, &start),
			false => compute_next_successor::<MIN>(state, grid),
		},
		|state| (end.0.abs_diff(state.position.0) + end.1.abs_diff(state.position.1)) as u64,
		|state| state.position == end && state.distance >= MIN,
	)
	.expect("Path to end exists");

	path.1
}

fn compute_neighbouring_successors<const MAX: u64>(
	state: &State,
	grid: &Matrix<u64>,
	start: &State,
) -> Vec<(State, u64)> {
	[directions::N, directions::S, directions::E, directions::W]
		.iter()
		.flat_map(|direction| {
			// 1. Find all valid neighbouring points relative to the current position.

			grid.move_in_direction(state.position, *direction)
				.map(|point| (point, *direction, *grid.get(point).expect("Point to exist")))
		})
		.filter(|(position, direction, _)| {
			// 2. Do not use any points which would cause us to backtrack.

			let is_direction_inverse =
				state.direction.0 == -direction.0 && state.direction.1 == -direction.1;

			!is_direction_inverse && *position != start.position
		})
		.flat_map(|(position, direction, weight)| {
			// 3. Create successors that are within max distance.

			let distance = match state.direction == direction {
				true => state.distance + 1,
				false => 1,
			};

			match distance <= MAX {
				true => {
					let next_state = State {
						position,
						direction,
						distance,
					};
					Some((next_state, weight))
				}
				false => None,
			}
		})
		.collect::<Vec<_>>()
}

fn compute_next_successor<const MIN: u64>(state: &State, grid: &Matrix<u64>) -> Vec<(State, u64)> {
	match grid.move_in_direction(state.position, state.direction) {
		Some(point) => {
			let weight = *grid.get(point).expect("Point to exist");
			let new_state = State {
				position: point,
				direction: state.direction,
				distance: state.distance + 1,
			};

			vec![(new_state, weight)]
		}
		None => Vec::with_capacity(0),
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
	position: (usize, usize),
	direction: (isize, isize),
	distance: u64,
}

#[cfg(test)]
mod tests_day_17 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#
		.trim();

		assert_eq!(part_1(input), 102);
	}

	#[test]
	fn part_01_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_1(input), 1044);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#
		.trim();

		assert_eq!(part_2(input), 94);
	}

	#[test]
	fn part_02_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_2(input), 1227);
	}
}

use rayon::prelude::*;
use std::collections::HashSet;
use std::fmt::Debug;

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let grid = parse(input);

	run_beam(
		Beam {
			position: Point { x: 0, y: 0 },
			direction: Direction::East,
		},
		&grid,
	)
}

fn part_2(input: &str) -> u64 {
	let grid = parse(input);

	// Generate starting positions along the walls of the grid.
	let start_beams = [
		(0..grid[0].len())
			.map(|x| Beam::new(x, 0, Direction::South))
			.collect::<Vec<_>>(),
		(0..grid.len())
			.map(|y| Beam::new(grid[0].len() - 1, y, Direction::West))
			.collect::<Vec<_>>(),
		(0..grid[0].len())
			.map(|x| Beam::new(x, grid.len() - 1, Direction::South))
			.collect::<Vec<_>>(),
		(0..grid.len())
			.map(|y| Beam::new(0, y, Direction::West))
			.collect::<Vec<_>>(),
	]
	.concat();

	start_beams
		.into_par_iter()
		.map(|b| run_beam(b, &grid))
		.max()
		.unwrap_or(0)
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
	input
		.lines()
		.map(|line| {
			line.chars()
				.map(|c| match c {
					'.' => Tile::Empty,
					'/' => Tile::Mirror(MirrorDirection::Forwards),
					'\\' => Tile::Mirror(MirrorDirection::Backwards),
					'|' => Tile::Splitter(SplitterDirection::Vertical),
					'-' => Tile::Splitter(SplitterDirection::Horizontal),
					_ => unreachable!("No other tile types should exist"),
				})
				.collect()
		})
		.collect()
}

fn run_beam(beam: Beam, grid: &[Vec<Tile>]) -> u64 {
	let mut beams = vec![beam];

	let mut visited = HashSet::new();

	while !beams.is_empty() {
		let next_beams = beams
			.into_iter()
			.filter_map(|beam| step_beam(beam, grid, &mut visited))
			.flatten()
			.collect::<Vec<_>>();

		beams = next_beams;
	}

	visited.iter().map(|v| v.0).collect::<HashSet<_>>().len() as u64
}

fn step_beam(
	beam: Beam,
	grid: &[Vec<Tile>],
	visited: &mut HashSet<(Point, Direction)>,
) -> Option<Vec<Beam>> {
	if visited.contains(&(beam.position, beam.direction)) {
		return None;
	}

	visited.insert((beam.position, beam.direction));

	let beams = match &grid[beam.position.y][beam.position.x] {
		Tile::Empty => vec![beam],
		Tile::Mirror(mirror_direction) => {
			let beam = match (beam.direction, mirror_direction) {
				(Direction::North, MirrorDirection::Forwards) => beam.rotate(Direction::East),
				(Direction::North, MirrorDirection::Backwards) => beam.rotate(Direction::West),
				(Direction::East, MirrorDirection::Forwards) => beam.rotate(Direction::North),
				(Direction::East, MirrorDirection::Backwards) => beam.rotate(Direction::South),
				(Direction::South, MirrorDirection::Forwards) => beam.rotate(Direction::West),
				(Direction::South, MirrorDirection::Backwards) => beam.rotate(Direction::East),
				(Direction::West, MirrorDirection::Forwards) => beam.rotate(Direction::South),
				(Direction::West, MirrorDirection::Backwards) => beam.rotate(Direction::North),
			};

			vec![beam]
		}
		Tile::Splitter(split_direction) => match split_direction {
			SplitterDirection::Vertical if beam.direction.is_horizontal() => {
				vec![
					beam.clone().rotate(Direction::North),
					beam.rotate(Direction::South),
				]
			}
			SplitterDirection::Horizontal if beam.direction.is_vertical() => {
				vec![
					beam.clone().rotate(Direction::East),
					beam.rotate(Direction::West),
				]
			}
			_ => vec![beam],
		},
	};

	let next_beams = beams
		.into_iter()
		.flat_map(|b| {
			b.direction.step(b.position, grid).map(|p| Beam {
				position: p,
				direction: b.direction,
			})
		})
		.collect::<Vec<_>>();

	match next_beams.is_empty() {
		true => None,
		false => Some(next_beams),
	}
}

#[derive(Clone, Debug)]
struct Beam {
	pub position: Point,
	pub direction: Direction,
}

impl Beam {
	fn new(x: usize, y: usize, direction: Direction) -> Self {
		Self {
			position: Point { x, y },
			direction,
		}
	}

	fn rotate(self, direction: Direction) -> Self {
		Self {
			position: self.position,
			direction,
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
	pub x: usize,
	pub y: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
	North,
	South,
	East,
	West,
}

impl Direction {
	fn step(&self, point: Point, grid: &[Vec<Tile>]) -> Option<Point> {
		match self {
			Direction::North => match point.y == 0 {
				true => None,
				false => Some(Point {
					x: point.x,
					y: point.y - 1,
				}),
			},
			Direction::South => match point.y == grid.len() - 1 {
				true => None,
				false => Some(Point {
					x: point.x,
					y: point.y + 1,
				}),
			},
			Direction::East => match point.x == grid[0].len() - 1 {
				true => None,
				false => Some(Point {
					x: point.x + 1,
					y: point.y,
				}),
			},
			Direction::West => match point.x == 0 {
				true => None,
				false => Some(Point {
					x: point.x - 1,
					y: point.y,
				}),
			},
		}
	}

	fn is_horizontal(&self) -> bool {
		match self {
			Direction::North | Direction::South => false,
			Direction::East | Direction::West => true,
		}
	}

	fn is_vertical(&self) -> bool {
		!self.is_horizontal()
	}
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Tile {
	Empty,
	Mirror(MirrorDirection),
	Splitter(SplitterDirection),
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum MirrorDirection {
	Forwards,
	Backwards,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum SplitterDirection {
	Vertical,
	Horizontal,
}

#[cfg(test)]
mod tests_day_16 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#
		.trim();

		assert_eq!(part_1(input), 46);
	}

	#[test]
	fn part_01_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_1(input), 7242);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#
		.trim();

		assert_eq!(part_2(input), 51);
	}

	#[test]
	fn part_02_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_2(input), 7572);
	}
}

use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

fn main() {
	let input = include_str!("part_1_input.txt");

	println!("Day 10, part 1: {}", part_1(input, false));
	println!("Day 10, part 2: {}", part_2(input, false));
}

fn part_1(input: &str, debug: bool) -> u64 {
	let (mut grid, start) = parse(input);

	let mut connection_points_to_check = VecDeque::new();
	connection_points_to_check.push_back(start);

	// Use breadth-first search to find node with longest distance from start.

	let mut max_distance = 0;
	while let Some(point) = connection_points_to_check.pop_front() {
		let (connection_points, distance) = {
			let node = get_grid_node(&grid, point);
			(find_connection_points(node), node.distance)
		};

		for point in connection_points {
			let visited_distance =
				visit_node(point, &mut grid, &mut connection_points_to_check, distance);
			max_distance = max_distance.max(visited_distance);
		}
	}

	if debug {
		draw_part_1_debug_grid(&grid, start);
	}

	max_distance
}

fn part_2(input: &str, debug: bool) -> u64 {
	let (mut grid, start) = parse(input);

	walk_main_loop(&mut grid, start);
	let contained_nodes = ray_trace_loop(&mut grid);

	if debug {
		draw_part_2_debug_grid(&grid);
	}

	contained_nodes
}

fn parse(input: &str) -> (Vec<Vec<Node>>, Point) {
	let mut start = Point::default();

	let mut grid: Vec<_> = input
		.lines()
		.enumerate()
		.map(|(y, line)| {
			line.chars()
				.enumerate()
				.map(|(x, c)| {
					let pipe = match c {
						'|' => Pipe::Vertical,
						'-' => Pipe::Horizontal,
						'L' => Pipe::TopRight,
						'J' => Pipe::TopLeft,
						'F' => Pipe::BottomRight,
						'7' => Pipe::BottomLeft,
						'.' => Pipe::Ground,
						'S' => Pipe::Start,
						_ => unreachable!("No other grid characters"),
					};

					if pipe == Pipe::Start {
						start.x = x;
						start.y = y;
					}

					Node {
						point: Point { x, y },
						pipe,
						visited: false,
						distance: 0,
						is_main_loop: false,
						contained: false,
					}
				})
				.collect()
		})
		.collect();

	replace_start_new(&mut grid, start);

	(grid, start)
}

fn replace_start_new(grid: &mut [Vec<Node>], start: Point) {
	let can_connect_top = match start.y.checked_sub(1) {
		Some(y) => match grid.get(y).and_then(|r| r.get(start.x)) {
			Some(node) => node.pipe.can_connect_bottom(),
			None => false,
		},
		None => false,
	};

	let can_connect_right = match start.x.checked_add(1) {
		Some(x) => match grid.get(start.y).and_then(|r| r.get(x)) {
			Some(node) => node.pipe.can_connect_left(),
			None => false,
		},
		None => false,
	};

	let can_connect_bottom = match start.y.checked_add(1) {
		Some(y) => match grid.get(y).and_then(|r| r.get(start.x)) {
			Some(node) => node.pipe.can_connect_top(),
			None => false,
		},
		None => false,
	};

	let can_connect_left = match start.x.checked_sub(1) {
		Some(x) => match grid.get(start.y).and_then(|r| r.get(x)) {
			Some(node) => node.pipe.can_connect_right(),
			None => false,
		},
		None => false,
	};

	let pipe = match (
		can_connect_top,
		can_connect_right,
		can_connect_bottom,
		can_connect_left,
	) {
		(true, true, false, false) => Pipe::TopRight,
		(true, false, true, false) => Pipe::Vertical,
		(true, false, false, true) => Pipe::TopLeft,
		(false, true, true, false) => Pipe::BottomRight,
		(false, true, false, true) => Pipe::Horizontal,
		(false, false, true, true) => Pipe::BottomLeft,
		_ => unreachable!("We are guaranteed to have 2 connections to start node"),
	};

	let start = get_grid_node_mut(grid, start);
	start.pipe = pipe;
}

fn get_grid_node(grid: &[Vec<Node>], point: Point) -> &Node {
	grid.get(point.y)
		.expect("Row to exist")
		.get(point.x)
		.expect("Column to exist")
}

fn get_grid_node_mut(grid: &mut [Vec<Node>], point: Point) -> &mut Node {
	grid.get_mut(point.y)
		.expect("Row to exist")
		.get_mut(point.x)
		.expect("Column to exist")
}

fn visit_node(
	point: Point,
	grid: &mut [Vec<Node>],
	queue: &mut VecDeque<Point>,
	distance: usize,
) -> u64 {
	let node = get_grid_node_mut(grid, point);

	if node.visited {
		return node.distance as u64;
	}

	node.distance = distance + 1;
	node.visited = true;
	queue.push_back(point);

	node.distance as u64
}

fn find_connection_points(node: &Node) -> [Point; 2] {
	let point = node.point;

	match node.pipe {
		Pipe::Vertical => [
			Point {
				x: point.x,
				y: point.y - 1,
			},
			Point {
				x: point.x,
				y: point.y + 1,
			},
		],
		Pipe::Horizontal => [
			Point {
				x: point.x - 1,
				y: point.y,
			},
			Point {
				x: point.x + 1,
				y: point.y,
			},
		],
		Pipe::TopRight => [
			Point {
				x: point.x,
				y: point.y - 1,
			},
			Point {
				x: point.x + 1,
				y: point.y,
			},
		],
		Pipe::TopLeft => [
			Point {
				x: point.x,
				y: point.y - 1,
			},
			Point {
				x: point.x - 1,
				y: point.y,
			},
		],
		Pipe::BottomRight => [
			Point {
				x: point.x,
				y: point.y + 1,
			},
			Point {
				x: point.x + 1,
				y: point.y,
			},
		],
		Pipe::BottomLeft => [
			Point {
				x: point.x,
				y: point.y + 1,
			},
			Point {
				x: point.x - 1,
				y: point.y,
			},
		],
		Pipe::Ground | Pipe::Start => unreachable!("These don't connect to anything"),
	}
}

/// Walk around the entire loop from the given start point, and mark
/// all nodes along the path as part of the main loop.
fn walk_main_loop(grid: &mut [Vec<Node>], start: Point) {
	let mut current = start;

	'outer: loop {
		let connection_points = {
			let node = get_grid_node(grid, current);
			find_connection_points(node)
		};

		for point in connection_points {
			if walk_node_new(grid, point) {
				current = point;
				continue 'outer;
			}
		}

		break;
	}
}

fn walk_node_new(grid: &mut [Vec<Node>], point: Point) -> bool {
	let node = get_grid_node_mut(grid, point);

	match node.is_main_loop {
		true => false,
		false => {
			node.is_main_loop = true;
			true
		}
	}
}

fn ray_trace_loop(grid: &mut [Vec<Node>]) -> u64 {
	grid.iter_mut()
		.map(|row| {
			let mut inside = false;
			let mut inside_count = 0;

			row.iter_mut().for_each(|node| {
				let pipe = node.pipe;

				if !node.is_main_loop && inside {
					node.contained = true;
					inside_count += 1;
				} else if node.is_main_loop
					&& (pipe == Pipe::TopLeft || pipe == Pipe::TopRight || pipe == Pipe::Vertical)
				{
					inside = !inside;
				}
			});

			inside_count
		})
		.sum()
}

fn draw_part_1_debug_grid(grid: &[Vec<Node>], start: Point) {
	for (y, node) in grid.iter().enumerate() {
		let line: Vec<_> = node
			.iter()
			.enumerate()
			.map(|(x, node)| {
				let value = match x == start.x && y == start.y {
					true => "S".to_string(),
					false => match node.pipe == Pipe::Ground {
						true => ".".to_string(),
						false => node.distance.to_string(),
					},
				};

				format!("{:>3}", value)
			})
			.collect();
		let line = line.join("");

		println!("{line}");
	}
}

fn draw_part_2_debug_grid(grid: &[Vec<Node>]) {
	for node in grid.iter() {
		let line: Vec<_> = node
			.iter()
			.map(|node| {
				let a = match node.is_main_loop {
					true => node.pipe.to_string(),
					false => match node.contained {
						true => "0".to_string(),
						false => " ".to_string(),
					},
				};
				format!("{:>3}", a)
			})
			.collect();
		let line = line.join("");

		println!("{line}");
	}
}

#[derive(Copy, Clone, Default)]
struct Point {
	pub x: usize,
	pub y: usize,
}

struct Node {
	pub point: Point,
	pub pipe: Pipe,
	pub visited: bool,
	pub distance: usize,
	pub is_main_loop: bool,
	pub contained: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pipe {
	Vertical,
	Horizontal,
	TopRight,
	TopLeft,
	BottomRight,
	BottomLeft,
	Ground,
	Start,
}

impl Pipe {
	fn can_connect_bottom(&self) -> bool {
		match self {
			Pipe::Vertical => true,
			Pipe::Horizontal => false,
			Pipe::TopRight => false,
			Pipe::TopLeft => false,
			Pipe::BottomRight => true,
			Pipe::BottomLeft => true,
			Pipe::Ground => false,
			Pipe::Start => false,
		}
	}

	fn can_connect_right(&self) -> bool {
		match self {
			Pipe::Vertical => false,
			Pipe::Horizontal => true,
			Pipe::TopRight => true,
			Pipe::TopLeft => false,
			Pipe::BottomRight => true,
			Pipe::BottomLeft => false,
			Pipe::Ground => false,
			Pipe::Start => false,
		}
	}

	fn can_connect_top(&self) -> bool {
		match self {
			Pipe::Vertical => true,
			Pipe::Horizontal => false,
			Pipe::TopRight => true,
			Pipe::TopLeft => true,
			Pipe::BottomRight => false,
			Pipe::BottomLeft => false,
			Pipe::Ground => false,
			Pipe::Start => false,
		}
	}

	fn can_connect_left(&self) -> bool {
		match self {
			Pipe::Vertical => false,
			Pipe::Horizontal => true,
			Pipe::TopRight => false,
			Pipe::TopLeft => true,
			Pipe::BottomRight => false,
			Pipe::BottomLeft => true,
			Pipe::Ground => false,
			Pipe::Start => false,
		}
	}
}

impl Display for Pipe {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Pipe::Vertical => f.write_str("│"),
			Pipe::Horizontal => f.write_str("─"),
			Pipe::TopRight => f.write_str("└"),
			Pipe::TopLeft => f.write_str("┘"),
			Pipe::BottomRight => f.write_str("┌"),
			Pipe::BottomLeft => f.write_str("┐"),
			Pipe::Ground => f.write_str("."),
			Pipe::Start => f.write_str("S"),
		}
	}
}

#[cfg(test)]
mod tests_day_10 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#
		.trim();

		assert_eq!(part_1(input, false), 8);
	}

	#[test]
	fn part_01_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_1(input, false), 6903);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#
		.trim();

		assert_eq!(part_2(input, false), 10);
	}

	#[test]
	fn part_02_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_2(input, false), 265);
	}
}

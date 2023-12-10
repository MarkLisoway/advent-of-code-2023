use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let lines: Vec<_> = input.lines().collect();
	let mut grid = vec![vec![Node::default(); lines[0].len()]; lines.len()];

	let mut start = Coordinates::default();
	for (i, line) in input.lines().enumerate() {
		for (j, char) in line.chars().enumerate() {
			let pipe = match char {
				'|' => Pipe::Vertical,
				'-' => Pipe::Horizontal,
				'L' => Pipe::TopRight,
				'J' => Pipe::TopLeft,
				'F' => Pipe::BottomRight,
				'7' => Pipe::BottomLeft,
				'.' => Pipe::Ground,
				'S' => Pipe::Start,
				_ => unreachable!(),
			};

			let node = Node {
				c: Coordinates { x: j, y: i },
				pipe,
				is_main_loop: false,
				visited: false,
				distance: 0,
				contained: false,
			};

			if pipe == Pipe::Start {
				start = node.c;
			}

			grid[i][j] = node;
		}
	}

	replace_start(start, &mut grid);

	let mut to_visit = VecDeque::new();
	to_visit.push_back(grid[start.y][start.x]);

	let mut max = 0;
	while let Some(node) = to_visit.pop_front() {
		let connections = find_connections(node.c, &grid);
		max = handle_node(node, connections[0], &mut grid, &mut to_visit).max(max);
		max = handle_node(node, connections[1], &mut grid, &mut to_visit).max(max);
	}

	// draw_grid(&grid, start);

	max
}

fn part_2(input: &str) -> u64 {
	let lines: Vec<_> = input.lines().collect();
	let mut grid = vec![vec![Node::default(); lines[0].len()]; lines.len()];

	let mut start = Coordinates::default();
	for (i, line) in input.lines().enumerate() {
		for (j, char) in line.chars().enumerate() {
			let pipe = match char {
				'|' => Pipe::Vertical,
				'-' => Pipe::Horizontal,
				'L' => Pipe::TopRight,
				'J' => Pipe::TopLeft,
				'F' => Pipe::BottomRight,
				'7' => Pipe::BottomLeft,
				'.' => Pipe::Ground,
				'S' => Pipe::Start,
				_ => unreachable!(),
			};

			let node = Node {
				c: Coordinates { x: j, y: i },
				pipe,
				is_main_loop: false,
				visited: false,
				distance: 0,
				contained: false,
			};

			if pipe == Pipe::Start {
				start = node.c;
			}

			grid[i][j] = node;
		}
	}

	replace_start(start, &mut grid);

	let mut current = start;
	loop {
		let connected = find_connections(current, &grid);
		let one = walk_node(connected[0], &mut grid);

		if one {
			current = connected[0];
			continue;
		}

		let two = walk_node(connected[1], &mut grid);

		if two {
			current = connected[1];
			continue;
		}

		break;
	}

	grid.iter()
		.flatten()
		.filter(|n| n.is_main_loop)
		.for_each(|node| {});

	let mut hits = vec![];

	let mut inside_count = 0;
	grid.iter().for_each(|row| {
		let mut inside = false;

		row.iter().for_each(|node| {
			let pipe = node.pipe;

			if !node.is_main_loop && inside {
				hits.push(node.c);
				inside_count += 1;
			} else if node.is_main_loop
				&& (pipe == Pipe::TopLeft || pipe == Pipe::TopRight || pipe == Pipe::Vertical)
			{
				inside = !inside;
			}
		});
	});

	for hit in hits {
		let node = grid.get_mut(hit.y).expect("").get_mut(hit.x).expect("");
		node.contained = true;
	}

	// draw_grid(&grid, start);

	inside_count
}

fn handle_node(
	parent: Node,
	c: Coordinates,
	grid: &mut [Vec<Node>],
	queue: &mut VecDeque<Node>,
) -> u64 {
	let node = grid.get_mut(c.y).unwrap().get_mut(c.x).unwrap();

	if node.visited {
		return node.distance as u64;
	}

	node.distance = parent.distance + 1;
	node.visited = true;
	queue.push_back(*node);

	node.distance as u64
}

fn walk_node(c: Coordinates, grid: &mut [Vec<Node>]) -> bool {
	let node = grid.get_mut(c.y).unwrap().get_mut(c.x).unwrap();

	if node.is_main_loop {
		return false;
	}

	node.is_main_loop = true;
	true
}

#[derive(Copy, Clone, Debug, Default)]
struct Coordinates {
	pub x: usize,
	pub y: usize,
}

#[derive(Copy, Clone, Debug, Default)]
struct Node {
	pub c: Coordinates,
	pub pipe: Pipe,
	pub is_main_loop: bool,
	pub visited: bool,
	pub distance: usize,
	pub contained: bool,
}

fn draw_grid(nodes: &[Vec<Node>], start: Coordinates) {
	for (y, node) in nodes.iter().enumerate() {
		let line: Vec<_> = node
			.iter()
			.enumerate()
			.map(|(x, n)| {
				let a = match n.is_main_loop {
					true => n.pipe.to_string(),
					false => match n.contained {
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

fn replace_start(start: Coordinates, grid: &mut [Vec<Node>]) {
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
		_ => unreachable!(),
	};

	let start = grid.get_mut(start.y).expect("").get_mut(start.x).expect("");
	start.pipe = pipe;
}

fn find_connections(c: Coordinates, grid: &[Vec<Node>]) -> [Coordinates; 2] {
	let node = grid[c.y][c.x];

	match node.pipe {
		Pipe::Vertical => [
			Coordinates { x: c.x, y: c.y - 1 },
			Coordinates { x: c.x, y: c.y + 1 },
		],
		Pipe::Horizontal => [
			Coordinates { x: c.x - 1, y: c.y },
			Coordinates { x: c.x + 1, y: c.y },
		],
		Pipe::TopRight => [
			Coordinates { x: c.x, y: c.y - 1 },
			Coordinates { x: c.x + 1, y: c.y },
		],
		Pipe::TopLeft => [
			Coordinates { x: c.x, y: c.y - 1 },
			Coordinates { x: c.x - 1, y: c.y },
		],
		Pipe::BottomRight => [
			Coordinates { x: c.x, y: c.y + 1 },
			Coordinates { x: c.x + 1, y: c.y },
		],
		Pipe::BottomLeft => [
			Coordinates { x: c.x, y: c.y + 1 },
			Coordinates { x: c.x - 1, y: c.y },
		],
		Pipe::Ground | Pipe::Start => unreachable!(),
	}
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

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
struct Cell {
	pub blocked_top: bool,
	pub blocked_right: bool,
	pub blocked_bottom: bool,
	pub blocked_left: bool,
	pub pipe: Pipe,
}

impl Display for Cell {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.pipe.fmt(f)
	}
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

impl Default for Pipe {
	fn default() -> Self {
		Self::Ground
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

		assert_eq!(part_1(input), 8);
	}

	#[test]
	fn part_01_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_1(input), 6903);
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

		assert_eq!(part_2(input), 10);
	}

	#[test]
	fn part_02_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_2(input), 265);
	}
}

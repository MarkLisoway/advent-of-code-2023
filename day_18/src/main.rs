use itertools::Itertools;

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> i64 {
	let instructions = parse_1(input);
	compute_area(&instructions)
}

fn part_2(input: &str) -> i64 {
	let instructions = parse_2(input);
	compute_area(&instructions)
}

fn parse_1(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|line| {
			let split = line.split(' ').collect::<Vec<_>>();
			let code = split[0];
			let distance = split[1].parse::<i64>().expect("Distance to parse");

			let direction = match code {
				"R" => Direction::Right,
				"D" => Direction::Down,
				"L" => Direction::Left,
				"U" => Direction::Up,
				_ => unreachable!("No other direction codes exist"),
			};

			Instruction {
				direction,
				distance,
			}
		})
		.collect()
}

fn parse_2(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|line| {
			let split = line.split(' ').collect::<Vec<_>>();
			let hex = split[2].to_string().replace(['(', ')', '#'], "");

			let distance = &hex[..5];
			let distance = i64::from_str_radix(distance, 16).expect("Distance to parse");
			let code = hex.chars().nth(5).expect("Code to exist");

			let direction = match code {
				'0' => Direction::Right,
				'1' => Direction::Down,
				'2' => Direction::Left,
				'3' => Direction::Up,
				_ => unreachable!("No other direction codes exist"),
			};

			Instruction {
				direction,
				distance,
			}
		})
		.collect()
}

fn compute_area(instructions: &[Instruction]) -> i64 {
	let mut current = Point { x: 0, y: 0 };
	let mut vertices = vec![Point { x: 0, y: 0 }];
	let mut total_distance = 0;

	instructions.iter().for_each(|instruction| {
		match instruction.direction {
			Direction::Right => current.x += instruction.distance,
			Direction::Down => current.y += instruction.distance,
			Direction::Left => current.x -= instruction.distance,
			Direction::Up => current.y -= instruction.distance,
		}

		vertices.push(current);
		total_distance += instruction.distance;
	});

	let area = vertices
		.iter()
		.tuple_windows()
		.fold(0, |acc, (p1, p2)| acc + (p1.x * p2.y) - (p1.y * p2.x));

	((total_distance + area) / 2) + 1
}

struct Instruction {
	direction: Direction,
	distance: i64,
}

enum Direction {
	Up,
	Down,
	Right,
	Left,
}

#[derive(Copy, Clone)]
struct Point {
	x: i64,
	y: i64,
}

#[cfg(test)]
mod tests_day_18 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#
		.trim();

		assert_eq!(part_1(input), 62);
	}

	#[test]
	fn part_01_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_1(input), 62365);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#
		.trim();

		assert_eq!(part_2(input), 952408144115);
	}

	#[test]
	fn part_02_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_2(input), 159485361249806);
	}
}

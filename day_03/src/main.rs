use std::collections::{HashMap, HashSet};
use std::rc::Rc;

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let engine = parse_input(input);

	engine
		.numbers
		.iter()
		.map(|n| {
			let adjacent_coordinates = n.generate_adjacent_coordinates();
			let next_to_symbol = adjacent_coordinates
				.iter()
				.any(|c| engine.symbols.contains(c));

			match next_to_symbol {
				true => n.value,
				false => 0,
			}
		})
		.sum()
}

fn part_2(input: &str) -> u64 {
	let engine = parse_input(input);

	engine
		.gears
		.iter()
		.map(|gear| {
			let adjacent = gear
				.generate_adjacent_coordinates()
				.iter()
				.flat_map(|c| {
					let number = engine.numbers_by_coord.get(c).map(|n| (n.id, n.value));

					number
				})
				.collect::<HashSet<(u64, u64)>>();

			let unique = adjacent.iter().cloned().collect::<HashSet<(u64, u64)>>();

			let has_two_unique = unique.len() == 2;

			let adjacent: Vec<_> = unique.into_iter().collect();

			match has_two_unique {
				true => adjacent[0].1 * adjacent[1].1,
				false => 0,
			}
		})
		.sum()
}

fn parse_input(input: &str) -> Engine {
	let mut numbers = vec![];
	let mut numbers_by_coord = HashMap::new();
	let mut symbols = HashSet::new();
	let mut gears = vec![];

	let mut id = 0;

	input.lines().enumerate().for_each(|(y, line)| {
		let line = format!("{line}.");
		let mut parsing_number = false;
		let mut number_start = 0;

		for (x, char) in line.chars().enumerate() {
			if char.is_ascii_digit() && !parsing_number {
				number_start = x;
				parsing_number = true;
			} else if !char.is_ascii_digit() && parsing_number {
				let number_end = x - 1;
				parsing_number = false;

				let num = &line[number_start..=number_end];
				let num = num.parse().expect("Number to parse successfully");

				id += 1;
				let number = Rc::new(Number {
					id,
					value: num,
					coordinates: (number_start..=number_end)
						.map(|x| Coordinate { x, y })
						.collect(),
				});

				numbers.push(number.clone());
				for coordinate in number.coordinates.iter() {
					numbers_by_coord.insert(coordinate.clone(), number.clone());
				}
			}

			if !char.is_ascii_digit() && char != '.' {
				symbols.insert(Coordinate { x, y });

				if char == '*' {
					gears.push(Coordinate { x, y });
				}
			}
		}
	});

	Engine {
		numbers,
		numbers_by_coord,
		symbols,
		gears,
	}
}

struct Engine {
	pub numbers: Vec<Rc<Number>>,
	pub numbers_by_coord: HashMap<Coordinate, Rc<Number>>,
	pub symbols: HashSet<Coordinate>,
	pub gears: Vec<Coordinate>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
	pub x: usize,
	pub y: usize,
}

impl Coordinate {
	fn generate_adjacent_coordinates(&self) -> Vec<Self> {
		let top = self.y.checked_sub(1).map(|y| Coordinate { x: self.x, y });
		let bottom = Some(Coordinate {
			x: self.x,
			y: self.y + 1,
		});

		let top_left = match (self.x.checked_sub(1), self.y.checked_sub(1)) {
			(Some(x), Some(y)) => Some(Coordinate { x, y }),
			_ => None,
		};

		let left = self.x.checked_sub(1).map(|x| Coordinate { x, y: self.y });

		let bottom_left = self
			.x
			.checked_sub(1)
			.map(|x| Coordinate { x, y: self.y + 1 });

		let top_right = self
			.y
			.checked_sub(1)
			.map(|y| Coordinate { x: self.x + 1, y });

		let right = Some(Coordinate {
			x: self.x + 1,
			y: self.y,
		});

		let bottom_right = Some(Coordinate {
			x: self.x + 1,
			y: self.y + 1,
		});

		vec![
			top,
			bottom,
			top_left,
			left,
			bottom_left,
			top_right,
			right,
			bottom_right,
		]
		.into_iter()
		.flatten()
		.collect()
	}
}

#[derive(Debug)]
struct Number {
	pub id: u64,
	pub value: u64,
	pub coordinates: Vec<Coordinate>,
}

impl Number {
	fn generate_adjacent_coordinates(&self) -> Vec<Coordinate> {
		self.coordinates
			.iter()
			.flat_map(|c| c.generate_adjacent_coordinates())
			.collect()
	}
}

#[cfg(test)]
mod tests_day_03 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#
		.trim();

		assert_eq!(part_1(input), 4361);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
    "#
		.trim();

		assert_eq!(part_2(input), 467835);
	}
}

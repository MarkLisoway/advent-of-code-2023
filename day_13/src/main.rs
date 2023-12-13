use rayon::prelude::*;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
	let input = include_str!("part_1_input.txt");

	let now = Instant::now();
	dbg!(part_1(input));
	dbg!(part_2(input));
	dbg!(Instant::now() - now);
}

fn part_1(input: &str) -> u64 {
	let lines = input.lines().collect::<Vec<_>>();

	let mut blocks = vec![];
	let mut start = 0;
	for (i, line) in lines.iter().enumerate() {
		if line.trim().is_empty() {
			blocks.push(&lines[start..i]);
			start = i + 1;
		}
	}
	blocks.push(&lines[start..lines.len()]);

	blocks
		.iter()
		.flat_map(|block| {
			let vertical = find_vertical_reflection(block);
			let horizontal = find_horizontal_reflection(block).map(|h| (h) * 100);

			if vertical.is_none() && horizontal.is_none() {
				draw_debug(block, horizontal, vertical);
			}

			vec![vertical, horizontal]
		})
		.flatten()
		.sum()
}

fn part_2(input: &str) -> u64 {
	let lines = input.lines().collect::<Vec<_>>();

	let mut blocks = vec![];
	let mut start = 0;
	for (i, line) in lines.iter().enumerate() {
		if line.trim().is_empty() {
			blocks.push(&lines[start..i]);
			start = i + 1;
		}
	}
	blocks.push(&lines[start..lines.len()]);

	blocks
		.par_iter()
		.flat_map(|block| {
			let vertical = find_flipped_vertical_reflection(block);
			let horizontal = find_flipped_horizontal_reflection(block).map(|h| (h) * 100);

			if vertical.is_none() && horizontal.is_none() {
				draw_debug(block, horizontal, vertical);
			}

			vec![vertical, horizontal]
		})
		.flatten()
		.sum()
}

fn draw_debug(input: &[&str], h: Option<u64>, v: Option<u64>) {
	for x in 0..input[0].len() {
		if let Some(v) = v {
			if v == x as u64 {
				print!(" v");
			} else {
				print!("  ");
			}
		}
	}

	for (y, line) in input.iter().enumerate() {
		if let Some(h) = h {
			if h / 100 == y as u64 {
				print!(">");
			}
		}
		println!();
		let chars = line.chars().map(|c| format!(" {c}")).collect::<Vec<_>>();
		let chars = chars.join("");
		println!(" {chars}");
	}

	println!();
	println!();
	println!();
}

fn find_vertical_reflection(input: &[&str]) -> Option<u64> {
	let a = input
		.iter()
		.map(|line| line.chars().collect::<Vec<_>>())
		.collect::<Vec<_>>();

	find_reflection(&a, None).map(|r| r as u64)
}

fn find_horizontal_reflection(input: &[&str]) -> Option<u64> {
	let line_len = input[0].len();

	let a = (0..line_len)
		.map(|x| {
			input
				.iter()
				.map(|line| line.chars().nth(x).expect("Character to exist"))
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	find_reflection(&a, None).map(|r| r as u64)
}

fn find_flipped_vertical_reflection(input: &[&str]) -> Option<u64> {
	let mut a = input
		.iter()
		.map(|line| line.chars().collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let original_reflection = find_reflection(&a, None);

	let len = a.first().unwrap().len();
	for y in 0..a.len() {
		for x in 0..len {
			flip_smudge(&mut a, x, y);
			let reflection = find_reflection(&a, original_reflection);
			flip_smudge(&mut a, x, y);

			if let Some(r) = reflection {
				return Some(r as u64);
			}
		}
	}

	None
}

fn find_flipped_horizontal_reflection(input: &[&str]) -> Option<u64> {
	let line_len = input[0].len();

	let mut a = (0..line_len)
		.map(|x| {
			input
				.iter()
				.map(|line| line.chars().nth(x).expect("Character to exist"))
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	let original_reflection = find_reflection(&a, None);

	let len = a.first().unwrap().len();
	for y in 0..a.len() {
		for x in 0..len {
			flip_smudge(&mut a, x, y);
			let reflection = find_reflection(&a, original_reflection);
			flip_smudge(&mut a, x, y);

			if let Some(r) = reflection {
				return Some(r as u64);
			}
		}
	}

	None
}

fn flip_smudge(grid: &mut [Vec<char>], x: usize, y: usize) {
	match grid[y][x] {
		'#' => grid[y][x] = '.',
		'.' => grid[y][x] = '#',
		_ => unreachable!(),
	}
}

fn find_reflection(lines: &[Vec<char>], except: Option<usize>) -> Option<usize> {
	let except = except.unwrap_or(0);

	let reflections = lines
		.iter()
		.map(|line| {
			(1..line.len())
				.filter(|midpoint| does_reflect(line, *midpoint) && *midpoint != except)
				.collect::<HashSet<_>>()
		})
		.reduce(|acc, h| acc.intersection(&h).cloned().collect())
		.expect("Final hash to exist");

	reflections.into_iter().next()
}

fn does_reflect(chars: &[char], midpoint: usize) -> bool {
	let mut min = midpoint - 1;
	let mut max = midpoint;

	let mut mirrors = true;
	loop {
		let left = chars[min];
		let right = chars[max];

		if left != right {
			mirrors = false;
			break;
		}

		if min == 0 || max == chars.len() - 1 {
			break;
		}

		min -= 1;
		max += 1;
	}

	mirrors
}

#[cfg(test)]
mod tests_day_13 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#
		.trim();

		assert_eq!(part_1(input), 405);
	}

	#[test]
	fn part_01_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_1(input), 36041);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#
		.trim();

		assert_eq!(part_2(input), 400);
	}

	#[test]
	fn part_02_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_2(input), 35915);
	}
}

use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let conditions = parse(input);

	conditions
		.par_iter()
		.map(|c| {
			let mut memo = HashMap::new();
			compute(&mut memo, c.conditions.as_bytes(), &c.damages, None)
		})
		.sum()
}

fn part_2(input: &str) -> u64 {
	let conditions = parse(input);

	conditions
		.par_iter()
		.map(|c| {
			let mut memo = HashMap::new();
			let conditions = (0..5).map(|_| &c.conditions).join("?");
			let damages = (0..5).flat_map(|_| &c.damages).cloned().collect::<Vec<_>>();

			compute(&mut memo, conditions.as_bytes(), &damages, None)
		})
		.sum()
}

fn parse(input: &str) -> Vec<SpringConditions> {
	input
		.lines()
		.map(|line| {
			let (springs, numbers) = line.split_once(' ').expect("Line to split");
			let numbers = numbers
				.split(',')
				.map(|n| n.parse::<usize>().expect("Number to parse"))
				.collect();

			SpringConditions {
				conditions: springs.to_string(),
				damages: numbers,
			}
		})
		.collect()
}

#[inline]
fn base_case(input: &[u8], numbers: &[usize], offset: Option<usize>) -> Option<u64> {
	if !input.is_empty() {
		return match numbers.is_empty() && offset.is_some() {
			true => Some(0),
			false => None,
		};
	}

	if numbers.is_empty() && offset.is_none() {
		return Some(1);
	}

	match (numbers.len(), offset) {
		(1, Some(o)) if o == numbers[0] => Some(1),
		_ => Some(0),
	}
}

fn compute(memo: &mut MemoCache, input: &[u8], numbers: &[usize], offset: Option<usize>) -> u64 {
	if let Some(base) = base_case(input, numbers, offset) {
		return base;
	}

	let memo_key = MemoKey {
		input_length: input.len(),
		offset,
		number_length: numbers.len(),
	};

	if let Some(memo_value) = memo.get(&memo_key) {
		return *memo_value;
	}

	let next = &input[1..];
	let permutations = match input[0] {
		b'?' => match offset {
			Some(o) => match o == numbers[0] {
				true => {
					compute(memo, next, numbers, Some(o + 1))
						+ compute(memo, next, &numbers[1..], None)
				}
				false => compute(memo, next, numbers, Some(o + 1)),
			},
			None => compute(memo, next, numbers, Some(1)) + compute(memo, next, numbers, None),
		},
		b'#' => match offset {
			Some(o) => compute(memo, next, numbers, Some(o + 1)),
			None => compute(memo, next, numbers, Some(1)),
		},
		b'.' => match offset {
			Some(o) => match o == numbers[0] {
				true => compute(memo, next, &numbers[1..], None),
				false => 0,
			},
			None => compute(memo, next, numbers, None),
		},
		_ => unreachable!(),
	};

	memo.insert(memo_key, permutations);

	permutations
}

#[derive(Debug)]
struct SpringConditions {
	pub conditions: String,
	pub damages: Vec<usize>,
}

type MemoCache = HashMap<MemoKey, u64>;

#[derive(Eq, PartialEq, Hash)]
struct MemoKey {
	pub input_length: usize,
	pub offset: Option<usize>,
	pub number_length: usize,
}

#[cfg(test)]
mod tests_day_12 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#
		.trim();

		assert_eq!(part_1(input), 21);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#
		.trim();

		assert_eq!(part_2(input), 525152);
	}
}

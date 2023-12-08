use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let network = parse(input);

	compute_traverse_distance("AAA".to_string(), &network, |e| e == "ZZZ")
}

fn part_2(input: &str) -> u64 {
	let network = parse(input);

	let start_nodes: Vec<_> = network
		.map
		.keys()
		.filter_map(|k| match k.ends_with('A') {
			true => Some(k.clone()),
			false => None,
		})
		.collect();

	let all_steps = start_nodes
		.into_par_iter()
		.map(|key| compute_traverse_distance(key, &network, |e| e.ends_with('Z')))
		.collect::<Vec<_>>();

	all_steps
		.into_iter()
		.reduce(num::integer::lcm)
		.expect("LCM to compute")
}

fn compute_traverse_distance<F: Fn(&str) -> bool>(
	mut key: String,
	network: &Network,
	break_check: F,
) -> u64 {
	for (i, step) in network.steps.iter().cycle().enumerate() {
		let next = network.map.get(&key).expect("Ket to exist");
		let next = match step {
			Step::Left => &next.0,
			Step::Right => &next.1,
		};

		if break_check(next) {
			return i as u64 + 1;
		}

		key = next.clone();
	}

	unreachable!("Previous loop is infinite unless return condition is met");
}

fn parse(input: &str) -> Network {
	let lines: Vec<_> = input.lines().collect();

	let steps = lines[0]
		.chars()
		.map(|c| match c {
			'L' => Step::Left,
			'R' => Step::Right,
			_ => unreachable!(),
		})
		.collect::<Vec<_>>();

	let lines = &lines[2..];

	let map = lines
		.iter()
		.map(|line| {
			let (key, value) = line.split_once(" = ").expect("Line to split");

			let value = value.replace(['(', ')'], "");
			let (left, right) = value.split_once(", ").expect("Value to split");

			(key.to_string(), (left.to_string(), right.to_string()))
		})
		.collect::<HashMap<_, _>>();

	Network { steps, map }
}

#[derive(Debug)]
struct Network {
	pub steps: Vec<Step>,
	pub map: HashMap<String, (String, String)>,
}

#[derive(Debug, Copy, Clone)]
enum Step {
	Left,
	Right,
}

#[cfg(test)]
mod tests_day_08 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#
		.trim();

		assert_eq!(part_1(input), 6);
	}

	#[test]
	fn part_01_test() {
		let input = include_str!("part_1_input.txt");
		assert_eq!(part_1(input), 12643);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#
		.trim();

		assert_eq!(part_2(input), 6);
	}

	#[test]
	fn part_02_test() {
		let input = include_str!("part_1_input.txt");
		assert_eq!(part_2(input), 13133452426987);
	}
}

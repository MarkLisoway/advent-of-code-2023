use rayon::prelude::*;

fn main() {
	let input = include_str!("part_1_input.txt");

	// let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".trim();

	// dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let inputs = input.split(",").collect::<Vec<_>>();

	inputs
		.par_iter()
		.map(|input| {
			let chars = input.chars().collect::<Vec<_>>();
			let mut hash: u64 = 0;

			chars.iter().for_each(|c| {
				let ascii = (*c as u8) as u64;
				hash += ascii;
				hash *= 17;
				hash %= 256;
			});

			hash
		})
		.sum()
}

fn part_2(input: &str) -> u64 {
	let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
	let inputs = input.split(",").collect::<Vec<_>>();

	inputs.iter().for_each(|input| {
		let (chars, number) = input
			.split_once(|c| c == '=' || c == '-')
			.expect("Line to split");
		let chars = chars.chars().collect::<Vec<_>>();
		let number = match number.is_empty() {
			true => None,
			false => Some(number.parse::<u64>().expect("Number to parse")),
		};

		let hash = hash_chars(&chars) as usize;
		let boxx = &mut boxes[hash];
		let box_index = boxx.iter().position(|e| e.chars == chars);

		match (box_index, number) {
			(Some(i), Some(n)) => {
				boxx[i].value = n;
			}
			(Some(i), None) => {
				boxx.remove(i);
			}
			(None, Some(n)) => boxx.push(Lens { chars, value: n }),
			(None, None) => {}
		}
	});

	boxes
		.iter()
		.enumerate()
		.map(|(i, boxx)| {
			boxx.iter()
				.enumerate()
				.map(|(j, e)| ((i + 1) * (j + 1) * (e.value as usize)) as u64)
				.sum::<u64>()
		})
		.sum()
}

fn hash_chars(chars: &[char]) -> u64 {
	let mut hash: u64 = 0;
	chars.iter().for_each(|c| {
		let ascii = (*c as u8) as u64;
		hash += ascii;
		hash *= 17;
		hash %= 256;
	});

	hash
}

#[derive(Clone, Debug)]
struct Lens {
	chars: Vec<char>,
	value: u64,
}

#[cfg(test)]
mod tests_day_15 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"

"#
		.trim();

		assert_eq!(part_1(input), 1);
	}

	#[test]
	fn part_02_example() {
		let input = r#"

"#
		.trim();

		assert_eq!(part_2(input), 1);
	}
}

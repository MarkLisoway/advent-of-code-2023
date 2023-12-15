fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let inputs = input.split(',').collect::<Vec<_>>();

	inputs.iter().map(|input| hash_str(input)).sum()
}

fn part_2(input: &str) -> u64 {
	let mut hashmap: Vec<Vec<Lens>> = vec![vec![]; 256];

	input.split(',').for_each(|input| {
		let (key, value) = input
			.split_once(|c| c == '=' || c == '-')
			.expect("Line to split");
		let value = match value.is_empty() {
			true => None,
			false => Some(value.parse::<u64>().expect("Number to parse")),
		};

		let hash = hash_str(key) as usize;
		let bucket = &mut hashmap[hash];
		let bucket_index = bucket.iter().position(|e| e.key == key);

		match (bucket_index, value) {
			(Some(i), Some(n)) => {
				// Have a previous value, and a new value, replace it.
				bucket[i].value = n;
			}
			(Some(i), None) => {
				// Have a previous value, but no new value, remove it.
				bucket.remove(i);
			}
			(None, Some(n)) => {
				// Have no previous value, but have new value, add it.
				bucket.push(Lens { key, value: n })
			}
			(None, None) => (),
		}
	});

	hashmap
		.iter()
		.enumerate()
		.map(|(bucket_index, bucket)| {
			bucket
				.iter()
				.enumerate()
				.map(|(value_index, lens)| {
					((bucket_index + 1) * (value_index + 1) * (lens.value as usize)) as u64
				})
				.sum::<u64>()
		})
		.sum()
}

fn hash_str(input: &str) -> u64 {
	input
		.chars()
		.map(|c| c as u64)
		.fold(0, |hash, v| ((hash + v) * 17) % 256)
}

#[derive(Clone, Debug)]
struct Lens<'a> {
	key: &'a str,
	value: u64,
}

#[cfg(test)]
mod tests_day_15 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".trim();

		assert_eq!(part_1(input), 1320);
	}

	#[test]
	fn part_01_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_1(input), 509167);
	}

	#[test]
	fn part_02_example() {
		let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".trim();

		assert_eq!(part_2(input), 145);
	}

	#[test]
	fn part_02_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_2(input), 259333);
	}
}

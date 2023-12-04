use std::collections::HashSet;
use std::rc::Rc;

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u32 {
	let cards = parse(input);
	cards
		.iter()
		.map(|card| {
			let base: u32 = 2;
			let matches = card.num_matches() as u32;

			match matches == 0 {
				true => 0,
				false => base.pow(matches - 1),
			}
		})
		.sum()
}

fn part_2(input: &str) -> u32 {
	let cards: Vec<_> = parse(input);

	let mut won_cards = vec![0_u32; cards.len()];

	cards.iter().enumerate().for_each(|(i, card)| {
		won_cards[i] += 1;

		let multiplier = won_cards[i];
		for j in 1..=card.num_matches() {
			won_cards[i + j] += multiplier;
		}
	});

	won_cards.iter().sum()
}

struct Card {
	pub winning_numbers: HashSet<u32>,
	pub our_numbers: HashSet<u32>,
}

impl Card {
	fn num_matches(&self) -> usize {
		self.winning_numbers.intersection(&self.our_numbers).count()
	}
}

fn parse(input: &str) -> Vec<Rc<Card>> {
	input
		.lines()
		.map(|line| {
			let (_, numbers) = line.split_once(':').expect("Game numbers to exist");

			let (winning, ours) = numbers.split_once('|').expect("Numbers to separate");

			let winning = parse_numbers(winning);
			let ours = parse_numbers(ours);

			Rc::new(Card {
				winning_numbers: winning,
				our_numbers: ours,
			})
		})
		.collect()
}

fn parse_numbers(input: &str) -> HashSet<u32> {
	input
		.split_whitespace()
		.map(|num| num.parse::<u32>().expect("Number to parse successfully"))
		.collect::<HashSet<u32>>()
}

#[cfg(test)]
mod tests_day_04 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#
		.trim();

		assert_eq!(part_1(input), 13);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#
		.trim();

		assert_eq!(part_2(input), 30);
	}
}

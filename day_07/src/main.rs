use std::collections::HashMap;
use std::ops::Range;

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let mut hands = parse(input);
	hands.sort_by_key(|c| c.power);

	sort_within_powers(&mut hands, 0);

	hands
		.iter()
		.rev()
		.enumerate()
		.map(|(i, hand)| hand.bid * (i as u64 + 1))
		.sum()
}

fn part_2(input: &str) -> u64 {
	let mut hands = parse_2(input);
	hands.sort_by_key(|c| c.power);

	sort_within_powers_2(&mut hands, 0);

	hands
		.iter()
		.rev()
		.enumerate()
		.map(|(i, hand)| hand.bid * (i as u64 + 1))
		.sum()
}

fn sort_within_powers(hands: &mut [Hand], card_index: usize) {
	if card_index > 4 {
		return;
	}

	let ranges = get_power_ranges(hands);

	for range in ranges {
		let hands = &mut hands[range];
		hands.sort_by_key(|c| c.cards[card_index]);

		let ranges = get_card_ranges(hands, card_index);

		for range in ranges {
			let hands = &mut hands[range];
			sort_within_powers(hands, card_index + 1);
		}
	}
}

fn get_power_ranges(hands: &[Hand]) -> Vec<Range<usize>> {
	let mut power = hands.first().expect("Hand to exist").power;
	let mut ranges = vec![];

	let mut start = 0;
	for (i, hand) in hands.iter().enumerate() {
		if hand.power != power {
			power = hand.power;
			ranges.push(start..i);
			start = i;
		} else if i == hands.len() - 1 {
			ranges.push(start..hands.len());
		}
	}

	ranges
}

fn get_card_ranges(hands: &[Hand], card_index: usize) -> Vec<Range<usize>> {
	let mut card = hands.first().expect("Hand to exist").cards[card_index];
	let mut ranges = vec![];

	let mut start = 0;
	for (i, hand) in hands.iter().enumerate() {
		if hand.cards[card_index] != card {
			card = hand.cards[card_index];
			ranges.push(start..i);
			start = i;
		} else if i == hands.len() - 1 {
			ranges.push(start..hands.len());
		}
	}

	ranges
}

fn sort_within_powers_2(hands: &mut [JHand], card_index: usize) {
	if card_index > 4 {
		return;
	}

	let ranges = get_power_ranges_2(hands);

	for range in ranges {
		let hands = &mut hands[range];
		hands.sort_by_key(|c| c.cards[card_index]);

		let ranges = get_card_ranges_2(hands, card_index);

		for range in ranges {
			let hands = &mut hands[range];
			sort_within_powers_2(hands, card_index + 1);
		}
	}
}

fn get_power_ranges_2(hands: &[JHand]) -> Vec<Range<usize>> {
	let mut power = hands.first().expect("Hand to exist").power;
	let mut ranges = vec![];

	let mut start = 0;
	for (i, hand) in hands.iter().enumerate() {
		if hand.power != power {
			power = hand.power;
			ranges.push(start..i);
			start = i;
		} else if i == hands.len() - 1 {
			ranges.push(start..hands.len());
		}
	}

	ranges
}

fn get_card_ranges_2(hands: &[JHand], card_index: usize) -> Vec<Range<usize>> {
	let mut card = hands.first().expect("Hand to exist").cards[card_index];
	let mut ranges = vec![];

	let mut start = 0;
	for (i, hand) in hands.iter().enumerate() {
		if hand.cards[card_index] != card {
			card = hand.cards[card_index];
			ranges.push(start..i);
			start = i;
		} else if i == hands.len() - 1 {
			ranges.push(start..hands.len());
		}
	}

	ranges
}

#[derive(Debug)]
struct Hand {
	cards: Vec<Card>,
	bid: u64,
	power: Power,
}

fn parse(input: &str) -> Vec<Hand> {
	input
		.lines()
		.map(|line| {
			let (cards, bid) = line.split_once(' ').expect("Line to split");

			let cards: Vec<Card> = cards
				.chars()
				.map(|c| match c {
					'A' => Card::A,
					'K' => Card::K,
					'Q' => Card::Q,
					'J' => Card::J,
					'T' => Card::T,
					'9' => Card::Nine,
					'8' => Card::Eight,
					'7' => Card::Seven,
					'6' => Card::Six,
					'5' => Card::Five,
					'4' => Card::Four,
					'3' => Card::Three,
					'2' => Card::Two,
					_ => unreachable!(),
				})
				.collect();
			let bid = bid.parse::<u64>().expect("Bid to parse");

			let mut card_count = HashMap::new();
			for card in cards.iter() {
				let entry = card_count.entry(*card).or_insert(0);
				*entry += 1;
			}

			let power = {
				let mut has_five = false;
				let mut has_four = false;
				let mut has_three = false;
				let mut has_two = false;
				let mut has_second_two = false;

				for (_, v) in card_count.iter() {
					let v = *v;
					if v == 5 {
						has_five = true;
					} else if v == 4 {
						has_four = true;
					} else if v == 3 {
						has_three = true;
					} else if v == 2 && has_two {
						has_second_two = true;
					} else if v == 2 {
						has_two = true;
					}
				}

				if has_five {
					Power::FiveOfAKind
				} else if has_four {
					Power::FourOfAKind
				} else if has_three && has_two {
					Power::FullHouse
				} else if has_three {
					Power::ThreeOfAKind
				} else if has_second_two {
					Power::TwoPair
				} else if has_two {
					Power::OnePair
				} else {
					Power::HighCard
				}
			};

			Hand { cards, bid, power }
		})
		.collect()
}

fn parse_2(input: &str) -> Vec<JHand> {
	input
		.lines()
		.map(|line| {
			let (cards, bid) = line.split_once(' ').expect("Line to split");

			let cards: Vec<JCard> = cards
				.chars()
				.map(|c| match c {
					'A' => JCard::A,
					'K' => JCard::K,
					'Q' => JCard::Q,
					'T' => JCard::T,
					'9' => JCard::Nine,
					'8' => JCard::Eight,
					'7' => JCard::Seven,
					'6' => JCard::Six,
					'5' => JCard::Five,
					'4' => JCard::Four,
					'3' => JCard::Three,
					'2' => JCard::Two,
					'J' => JCard::J,
					_ => unreachable!(),
				})
				.collect();
			let bid = bid.parse::<u64>().expect("Bid to parse");

			let mut card_count = HashMap::new();
			cards.iter().for_each(|card| {
				card_count.entry(*card).and_modify(|e| *e += 1).or_insert(1);
			});

			let has_joker = card_count.get(&JCard::J).is_some();
			let joker_count = *card_count.get(&JCard::J).unwrap_or(&0);

			let power = match card_count.len() {
				1 => Power::FiveOfAKind,
				2 => {
					if has_joker {
						Power::FiveOfAKind
					} else if card_count.values().any(|v| *v == 4) {
						Power::FourOfAKind
					} else {
						Power::FullHouse
					}
				}
				3 => {
					if has_joker {
						let is_two_pair = card_count
							.iter()
							.filter(|c| c.0 != &JCard::J)
							.map(|c| *c.1)
							.all(|c| c == 2);

						let reached_four = card_count
							.iter()
							.filter(|c| c.0 != &JCard::J)
							.any(|c| *c.1 + joker_count == 4);

						if reached_four {
							Power::FourOfAKind
						} else if is_two_pair {
							Power::FullHouse
						} else {
							Power::ThreeOfAKind
						}
					} else if card_count.values().any(|v| *v == 3) {
						Power::ThreeOfAKind
					} else {
						Power::TwoPair
					}
				}
				4 => {
					if has_joker {
						Power::ThreeOfAKind
					} else {
						Power::OnePair
					}
				}
				5 => {
					if has_joker {
						Power::OnePair
					} else {
						Power::HighCard
					}
				}
				_ => unreachable!(),
			};

			JHand { cards, bid, power }
		})
		.collect()
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Card {
	A,
	K,
	Q,
	J,
	T,
	Nine,
	Eight,
	Seven,
	Six,
	Five,
	Four,
	Three,
	Two,
}

#[derive(Clone, Debug)]
struct JHand {
	cards: Vec<JCard>,
	bid: u64,
	power: Power,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum JCard {
	A,
	K,
	Q,
	T,
	Nine,
	Eight,
	Seven,
	Six,
	Five,
	Four,
	Three,
	Two,
	J,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Power {
	FiveOfAKind,
	FourOfAKind,
	FullHouse,
	ThreeOfAKind,
	TwoPair,
	OnePair,
	HighCard,
}

#[cfg(test)]
mod tests_day_07 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#
		.trim();

		assert_eq!(part_1(input), 6440);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#
		.trim();

		assert_eq!(part_2(input), 5905);
	}
}

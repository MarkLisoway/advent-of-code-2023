use itertools::Itertools;

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let mut hands = parse(input, false);
	sort_hands(&mut hands);
	compute_hand_value_sum(&hands)
}

fn part_2(input: &str) -> u64 {
	let mut hands = parse(input, true)
		.into_iter()
		.map(|mut hand| {
			hand.power = compute_most_powerful_joker(&hand.cards);

			hand
		})
		.collect::<Vec<_>>();

	sort_hands(&mut hands);
	compute_hand_value_sum(&hands)
}

fn sort_hands(hands: &mut [Hand]) {
	hands.sort_by_key(|h| (h.power, h.cards_rank.clone()));
}

fn compute_hand_value_sum(hands: &[Hand]) -> u64 {
	hands
		.iter()
		.rev()
		.enumerate()
		.map(|(i, hand)| hand.bid * (i as u64 + 1))
		.sum()
}

fn compute_most_powerful_joker(cards: &[Card]) -> Power {
	let (card, _) = cards
		.iter()
		.filter(|c| **c != Card::Joker)
		.sorted()
		.group_by(|c| **c)
		.into_iter()
		.map(|(c, g)| (c, g.count()))
		.max_by_key(|(_, c)| *c)
		.unwrap_or((Card::Ace, 5));

	let replaced_joker = cards
		.iter()
		.map(|c| match *c {
			Card::Joker => card,
			_ => *c,
		})
		.collect::<Vec<_>>();

	compute_power(&replaced_joker)
}

fn parse(input: &str, is_joker: bool) -> Vec<Hand> {
	input
		.lines()
		.map(|line| {
			let (cards, bid) = line.split_once(' ').expect("Line to split");

			let cards = cards
				.chars()
				.map(|c| match c {
					'A' => Card::Ace,
					'K' => Card::King,
					'Q' => Card::Queen,
					'J' => match is_joker {
						true => Card::Joker,
						false => Card::Jack,
					},
					'T' => Card::Ten,
					'9' => Card::Nine,
					'8' => Card::Eight,
					'7' => Card::Seven,
					'6' => Card::Six,
					'5' => Card::Five,
					'4' => Card::Four,
					'3' => Card::Three,
					'2' => Card::Two,
					_ => unreachable!("No other cards exist"),
				})
				.collect::<Vec<_>>();

			let cards_rank = compute_cards_rank(&cards);
			let power = compute_power(&cards);

			Hand {
				cards,
				cards_rank,
				bid: bid.parse().expect("Bid to parse"),
				power,
			}
		})
		.collect()
}

fn compute_power(cards: &[Card]) -> Power {
	let cards: Vec<_> = cards
		.iter()
		.sorted()
		.group_by(|c| **c)
		.into_iter()
		.map(|(_, g)| g.count())
		.sorted()
		.collect();

	let unique_cards = cards.len();

	match unique_cards {
		1 => Power::FiveOfAKind,
		2 => match cards[1] == 4 {
			true => Power::FourOfAKind,
			false => Power::FullHouse,
		},
		3 => match cards[2] == 3 {
			true => Power::ThreeOfAKind,
			false => Power::TwoPair,
		},
		4 => Power::OnePair,
		5 => Power::HighCard,
		_ => unreachable!("Cannot have more than 5 unique cards"),
	}
}

fn compute_cards_rank(cards: &[Card]) -> String {
	cards
		.iter()
		.map(|c| match c {
			Card::Ace => "0",
			Card::King => "1",
			Card::Queen => "2",
			Card::Jack => "4",
			Card::Ten => "5",
			Card::Nine => "6",
			Card::Eight => "7",
			Card::Seven => "8",
			Card::Six => "9",
			Card::Five => "A",
			Card::Four => "B",
			Card::Three => "C",
			Card::Two => "D",
			Card::Joker => "E",
		})
		.join("")
}

#[derive(Debug)]
struct Hand {
	cards: Vec<Card>,
	cards_rank: String,
	bid: u64,
	power: Power,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
	Ace,
	King,
	Queen,
	Jack,
	Ten,
	Nine,
	Eight,
	Seven,
	Six,
	Five,
	Four,
	Three,
	Two,
	Joker,
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
	fn part_01_test() {
		let input = include_str!("part_1_input.txt");
		assert_eq!(249726565, part_1(input));
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

	#[test]
	fn part_02_test() {
		let input = include_str!("part_1_input.txt");
		assert_eq!(251135960, part_2(input));
	}
}

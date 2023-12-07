use rayon::prelude::*;

fn main() {
	let input = include_str!("part_1_input.txt");

	dbg!(part_1(input));
	dbg!(part_2(input));
}

fn part_1(input: &str) -> u64 {
	let garden = parse(input);

	let mut distances = vec![0_u64; garden.seeds.len()];
	for (i, seed) in garden.seeds.iter().enumerate() {
		let location = compute_seed_location(*seed, &garden);

		distances[i] = location;
	}

	*distances.iter().min().expect("Min value to exist")
}

fn part_2(input: &str) -> u64 {
	let garden = parse(input);

	let many_seeds: Vec<_> = input
		.lines()
		.next()
		.expect("First line to exist")
		.split_once(':')
		.expect("Seed line should split")
		.1
		.split_ascii_whitespace()
		.collect();

	let mut seed_ranges = vec![];
	for (i, _) in many_seeds.iter().enumerate() {
		if i % 2 == 1 {
			// Skip odd entries as they are the counts
			continue;
		}

		let start = many_seeds[i].parse::<u64>().expect("Seed start to parse");
		let count = many_seeds[i + 1]
			.parse::<u64>()
			.expect("Seed count to parse");

		seed_ranges.push((start, count));
	}

	seed_ranges
		.into_par_iter()
		.map(|(start, count)| {
			let mut min_distance = u64::MAX;

			for seed in start..(start + count) {
				let location = compute_seed_location(seed, &garden);

				if location < min_distance {
					min_distance = location;
				}
			}

			min_distance
		})
		.min()
		.expect("Min value to exist")
}

fn parse(input: &str) -> Garden {
	let lines: Vec<&str> = input.lines().collect();

	let seeds: Vec<u64> = lines[0]
		.split_once(':')
		.expect("Seed line should split")
		.1
		.split_ascii_whitespace()
		.map(|n| n.parse().expect("Seed number to parse"))
		.collect();

	let mut block_start = 3;
	let mut blocks = vec![];
	for i in 3..lines.len() {
		let line = lines[i];
		if line.is_empty() || i == lines.len() - 1 {
			let block = &lines[block_start..i];
			blocks.push(block);

			block_start = i + 2;
		}
	}

	Garden {
		seeds,
		seed_to_soil: GardeningMap::parse(blocks[0]),
		soil_to_fertilizer: GardeningMap::parse(blocks[1]),
		fertilizer_to_water: GardeningMap::parse(blocks[2]),
		water_to_light: GardeningMap::parse(blocks[3]),
		light_to_temperature: GardeningMap::parse(blocks[4]),
		temperature_to_humidity: GardeningMap::parse(blocks[5]),
		humidity_to_location: GardeningMap::parse(blocks[6]),
	}
}

fn compute_seed_location(seed: u64, garden: &Garden) -> u64 {
	let soil = garden.seed_to_soil.get_dest(seed);
	let fertilizer = garden.soil_to_fertilizer.get_dest(soil);
	let water = garden.fertilizer_to_water.get_dest(fertilizer);
	let light = garden.water_to_light.get_dest(water);
	let temperature = garden.light_to_temperature.get_dest(light);
	let humidity = garden.temperature_to_humidity.get_dest(temperature);

	garden.humidity_to_location.get_dest(humidity)
}

struct Garden {
	pub seeds: Vec<u64>,
	pub seed_to_soil: GardeningMap,
	pub soil_to_fertilizer: GardeningMap,
	pub fertilizer_to_water: GardeningMap,
	pub water_to_light: GardeningMap,
	pub light_to_temperature: GardeningMap,
	pub temperature_to_humidity: GardeningMap,
	pub humidity_to_location: GardeningMap,
}

#[derive(Debug)]
struct GardeningMap {
	inner: Vec<Range>,
}

impl GardeningMap {
	fn parse(input: &[&str]) -> Self {
		let ranges = input
			.iter()
			.map(|line| {
				let parts: Vec<&str> = line.split_ascii_whitespace().collect();

				let source_start = parts[1].parse::<u64>().expect("Start to parse");
				let destination_start = parts[0].parse::<u64>().expect("End to parse");
				let range = parts[2].parse::<u64>().expect("Range to parse");

				Range {
					source_start,
					destination_start,
					range,
				}
			})
			.collect();

		Self { inner: ranges }
	}

	fn get_dest(&self, source: u64) -> u64 {
		for range in self.inner.iter().rev() {
			if let Some(d) = range.is_in_range(source) {
				return d;
			}
		}

		source
	}
}

#[derive(Debug)]
struct Range {
	pub source_start: u64,
	pub destination_start: u64,
	pub range: u64,
}

impl Range {
	fn is_in_range(&self, source: u64) -> Option<u64> {
		if source >= self.source_start && source < self.source_start + self.range {
			let diff = source - self.source_start;
			return Some(self.destination_start + diff);
		}

		None
	}
}

#[cfg(test)]
mod tests_day_05 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#
		.trim();

		assert_eq!(part_1(input), 35);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#
		.trim();

		assert_eq!(part_2(input), 46);
	}
}

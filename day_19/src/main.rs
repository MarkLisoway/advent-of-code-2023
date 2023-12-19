use fxhash::FxHashMap;
use std::ops::RangeInclusive;
use std::time::Instant;

fn main() {
	let input = include_str!("part_1_input.txt");

	let now = Instant::now();
	dbg!(part_1(input));
	dbg!(part_2(input));
	dbg!(Instant::now() - now);
}

fn part_1(input: &str) -> u64 {
	let (workflows, parts) = parse(input);

	parts
		.iter()
		.filter(|part| {
			let mut workflow = workflows.get("in").expect("Initial workflow to exist");

			'outer: loop {
				for action in workflow {
					match action {
						Action::LessThan(code, value, action) => {
							match get_value_from_code(part, code) < *value {
								true => match action {
									CompareAction::SendTo(next_workflow) => {
										workflow = workflows
											.get(next_workflow)
											.expect("Next workflow to exist");
										continue 'outer;
									}
									CompareAction::Accept => return true,
									CompareAction::Reject => return false,
								},
								false => continue,
							}
						}
						Action::GreaterThan(code, value, action) => {
							match get_value_from_code(part, code) > *value {
								true => match action {
									CompareAction::SendTo(next_workflow) => {
										workflow = workflows
											.get(next_workflow)
											.expect("Next workflow to exist");
										continue 'outer;
									}
									CompareAction::Accept => return true,
									CompareAction::Reject => return false,
								},
								false => continue,
							}
						}
						Action::Accept => return true,
						Action::Reject => return false,
						Action::Goto(next_workflow) => {
							workflow = workflows
								.get(next_workflow)
								.expect("Next workflow to exist");
							continue 'outer;
						}
					}
				}
			}
		})
		.fold(0, |acc, part| acc + part.a + part.m + part.s + part.x)
}

fn part_2(input: &str) -> u64 {
	let (workflows, _) = parse(input);

	let range = PartRange::default();
	let state = ComputeState {
		workflow: "in".to_string(),
		action_index: 0,
	};

	compute_accepted(range, state, &workflows)
}

fn compute_accepted(
	range: PartRange,
	state: ComputeState,
	workflows: &FxHashMap<String, Vec<Action>>,
) -> u64 {
	let workflow = workflows.get(&state.workflow).expect("Workflow to exist");
	let action = &workflow[state.action_index];

	match action {
		Action::LessThan(code, value, action) => {
			let (left, right) = range.split(code, *value - 1);

			let left_result = match action {
				CompareAction::SendTo(next_workflow) => {
					let state = ComputeState {
						workflow: next_workflow.to_string(),
						action_index: 0,
					};
					compute_accepted(left, state, workflows)
				}
				CompareAction::Accept => left.permutations(),
				CompareAction::Reject => 0,
			};

			let right_result = {
				let state = ComputeState {
					workflow: state.workflow,
					action_index: state.action_index + 1,
				};

				compute_accepted(right, state, workflows)
			};

			left_result + right_result
		}
		Action::GreaterThan(code, value, action) => {
			let (left, right) = range.split(code, *value);

			let left_result = {
				let state = ComputeState {
					workflow: state.workflow,
					action_index: state.action_index + 1,
				};

				compute_accepted(left, state, workflows)
			};

			let right_result = match action {
				CompareAction::SendTo(next_workflow) => {
					let state = ComputeState {
						workflow: next_workflow.to_string(),
						action_index: 0,
					};
					compute_accepted(right, state, workflows)
				}
				CompareAction::Accept => right.permutations(),
				CompareAction::Reject => 0,
			};

			left_result + right_result
		}
		Action::Accept => range.permutations(),
		Action::Reject => 0,
		Action::Goto(code) => {
			let state = ComputeState {
				workflow: code.to_string(),
				action_index: 0,
			};
			compute_accepted(range, state, workflows)
		}
	}
}

#[derive(Clone, Debug)]
struct PartRange {
	a: RangeInclusive<u64>,
	m: RangeInclusive<u64>,
	s: RangeInclusive<u64>,
	x: RangeInclusive<u64>,
}

impl PartRange {
	fn permutations(&self) -> u64 {
		(*self.a.end() - *self.a.start() + 1)
			* (*self.m.end() - *self.m.start() + 1)
			* (*self.s.end() - *self.s.start() + 1)
			* (*self.x.end() - *self.x.start() + 1)
	}

	fn split(self, code: &str, value: u64) -> (Self, Self) {
		let range = match code {
			"a" => &self.a,
			"m" => &self.m,
			"s" => &self.s,
			"x" => &self.x,
			_ => unreachable!("No other codes exist"),
		};

		let left = *range.start()..=value;
		let right = (value + 1)..=*range.end();

		let mut lower = self.clone();
		let mut upper = self.clone();

		match code {
			"a" => {
				lower.a = left;
				upper.a = right;
			}
			"m" => {
				lower.m = left;
				upper.m = right;
			}
			"s" => {
				lower.s = left;
				upper.s = right;
			}
			"x" => {
				lower.x = left;
				upper.x = right;
			}
			_ => unreachable!("No other codes exist"),
		}

		(lower, upper)
	}
}

impl Default for PartRange {
	fn default() -> Self {
		Self {
			a: 1..=4000,
			m: 1..=4000,
			s: 1..=4000,
			x: 1..=4000,
		}
	}
}

struct ComputeState {
	workflow: String,
	action_index: usize,
}

fn get_value_from_code(part: &Part, code: &str) -> u64 {
	match code {
		"a" => part.a,
		"m" => part.m,
		"s" => part.s,
		"x" => part.x,
		_ => unreachable!("No other codes exist"),
	}
}

fn parse(input: &str) -> (FxHashMap<String, Vec<Action>>, Vec<Part>) {
	let lines = input.lines().collect::<Vec<_>>();

	let blank_line = lines
		.iter()
		.position(|line| line.is_empty())
		.expect("A blank line to exist");

	let workflows = lines[..blank_line]
		.iter()
		.map(|line| {
			let parts = line.split('{').collect::<Vec<_>>();

			let code = parts[0];
			let steps = &parts[1][..(parts[1].len() - 1)];
			let steps = steps
				.split(',')
				.map(|step| match step.contains(':') {
					true => {
						let (code, action) = step
							.split_once(|c| c == '<' || c == '>')
							.expect("Split to succeed");
						let (num, goto) = action.split_once(':').expect("Action to parse");
						let num = num.parse().expect("Number to parse");

						let action = match goto {
							"A" => CompareAction::Accept,
							"R" => CompareAction::Reject,
							_ => CompareAction::SendTo(goto.to_string()),
						};

						match step.contains('<') {
							true => Action::LessThan(code.to_string(), num, action),
							false => Action::GreaterThan(code.to_string(), num, action),
						}
					}
					false if step.starts_with('A') => Action::Accept,
					false if step.starts_with('R') => Action::Reject,
					false => Action::Goto(step.to_string()),
				})
				.collect::<Vec<_>>();

			(code.to_string(), steps)
		})
		.collect::<FxHashMap<_, _>>();

	let parts = lines[(blank_line + 1)..]
		.iter()
		.map(|line| {
			let mut part = Part::default();

			line[1..(line.len() - 1)].split(',').for_each(|rating| {
				let (code, num) = rating.split_once('=').expect("Rating to split");
				let num = num.parse().expect("Number to parse");

				match code {
					"a" => part.a = num,
					"m" => part.m = num,
					"s" => part.s = num,
					"x" => part.x = num,
					_ => unreachable!("No other rating codes exist"),
				}
			});

			part
		})
		.collect::<Vec<_>>();

	(workflows, parts)
}

#[derive(Debug)]
enum Action {
	LessThan(String, u64, CompareAction),
	GreaterThan(String, u64, CompareAction),
	Accept,
	Reject,
	Goto(String),
}

#[derive(Debug)]
enum CompareAction {
	SendTo(String),
	Accept,
	Reject,
}

#[derive(Debug, Default)]
struct Part {
	a: u64,
	m: u64,
	s: u64,
	x: u64,
}

#[cfg(test)]
mod tests_day_19 {
	use super::*;

	#[test]
	fn part_01_example() {
		let input = r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#
		.trim();

		assert_eq!(part_1(input), 19114);
	}

	#[test]
	fn part_01_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_1(input), 472630);
	}

	#[test]
	fn part_02_example() {
		let input = r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#
		.trim();

		assert_eq!(part_2(input), 167409079868000);
	}

	#[test]
	fn part_02_test() {
		let input = include_str!("part_1_input.txt");

		assert_eq!(part_2(input), 116738260946855);
	}
}

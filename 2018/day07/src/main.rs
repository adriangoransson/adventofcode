use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{stdin, Read};
use std::str::FromStr;

type Step = u8;

struct Config {
    num_workers: usize,
    alpha_zero: u8,
}

impl Config {
    fn part1() -> Self {
        Config {
            num_workers: 1,
            alpha_zero: b'A', // doesn't matter
        }
    }

    fn part2() -> Self {
        Config {
            num_workers: 5,
            alpha_zero: b'A' - 60,
        }
    }
}

struct Instruction {
    name: Step,
    blocks: Step,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let name = iter.nth(1).unwrap().as_bytes()[0];
        let blocks = iter.nth(5).unwrap().as_bytes()[0];

        Ok(Instruction { name, blocks })
    }
}

#[derive(Clone, Debug)]
enum WorkerState {
    Working,
    Done { step: Step },
}

fn parse_dependencies(input: &str) -> HashMap<Step, BTreeSet<Step>> {
    let mut dependencies = HashMap::new();
    input
        .lines()
        .flat_map(Instruction::from_str)
        .for_each(|Instruction { name, blocks }| {
            dependencies.entry(name).or_insert_with(BTreeSet::new);
            dependencies
                .entry(blocks)
                .or_insert_with(BTreeSet::new)
                .insert(name);
        });

    dependencies
}

fn step_sequence(
    Config {
        num_workers,
        alpha_zero,
    }: Config,
    dependencies: &HashMap<Step, BTreeSet<Step>>,
) -> (String, u32) {
    let steps: BTreeSet<Step> = dependencies.keys().copied().collect();

    let mut visited: HashSet<Step> = HashSet::with_capacity(steps.len());
    let mut worked_on: HashSet<Step> = HashSet::with_capacity(num_workers);
    let mut visits = Vec::with_capacity(steps.len());

    // Work queue.
    let mut workers: Vec<Vec<WorkerState>> = vec![Vec::new(); num_workers];

    let mut seconds = 0;
    while steps.len() != visited.len() {
        // First clear off all work.
        workers.iter_mut().for_each(|worker| match worker.pop() {
            Some(WorkerState::Working) | None => (),
            Some(WorkerState::Done { step }) => {
                visited.insert(step);
                worked_on.remove(&step);
            }
        });

        // Then reiterate to add work to everyone idle.
        // Needs to be done in a separate loop,
        // or idle workers in the front will miss available steps released after them.
        workers
            .iter_mut()
            .filter(|worker_queue| worker_queue.is_empty())
            .for_each(|worker| {
                let visited_or_processing = |s| visited.contains(s) || worked_on.contains(s);
                let dependencies_cleared = |s| dependencies[s].iter().all(|i| visited.contains(i));

                if let Some(&step) = steps
                    .iter()
                    .find(|s| !visited_or_processing(s) && dependencies_cleared(s))
                {
                    let wait = step - alpha_zero;

                    worker.push(WorkerState::Done { step });
                    (0..wait).for_each(|_| worker.push(WorkerState::Working));

                    worked_on.insert(step);
                    visits.push(step);
                }
            });

        seconds += 1;
    }

    (String::from_utf8(visits).unwrap(), seconds - 1)
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let dependencies = parse_dependencies(&input);

    let (visits, _) = step_sequence(Config::part1(), &dependencies);
    let (_, seconds) = step_sequence(Config::part2(), &dependencies);

    println!("Part 1. Step order: {}", visits);
    println!("Part 2. Seconds: {}", seconds);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dependencies() -> HashMap<Step, BTreeSet<Step>> {
        parse_dependencies(
            "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.",
        )
    }

    #[test]
    fn test_part1() {
        let (visits, _) = step_sequence(Config::part1(), &dependencies());
        assert_eq!("CABDFE", visits);
    }

    #[test]
    fn test_part2() {
        let config = Config {
            num_workers: 2,
            alpha_zero: b'A',
        };

        let (_, seconds) = step_sequence(config, &dependencies());
        assert_eq!(15, seconds);
    }
}

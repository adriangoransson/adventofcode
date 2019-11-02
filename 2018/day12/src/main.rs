use std::io::{stdin, Read};
use std::str::FromStr;
use std::{
    collections::{HashMap, VecDeque},
    iter::Iterator,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Pot {
    Empty,
    Plant,
}

impl Pot {
    fn from_u8(b: u8) -> Self {
        match b {
            b'#' => Pot::Plant,
            _ => Pot::Empty,
        }
    }
}

#[derive(Debug)]
struct Plants {
    pots: Vec<Pot>,
    rules: HashMap<Vec<Pot>, Pot>,
    zero_index: u64,
}

impl Plants {
    fn evolve(&self, generations: u64) -> Self {
        let mut pots = VecDeque::from(self.pots.clone());
        let mut zero_index = self.zero_index;

        for _ in 0..generations {
            // Make some space
            while pots.iter().take(5).any(|p| *p == Pot::Plant) {
                pots.push_front(Pot::Empty);
                zero_index += 1;
            }

            while pots.iter().rev().take(5).any(|p| *p == Pot::Plant) {
                pots.push_back(Pot::Empty);
            }

            let old = pots.clone();

            for (i, pot) in pots.iter_mut().enumerate().skip(2) {
                let slice: Vec<Pot> = old.iter().skip(i - 2).take(5).copied().collect();

                *pot = if let Some(Pot::Plant) = self.rules.get(&slice) {
                    Pot::Plant
                } else {
                    Pot::Empty
                };
            }
        }

        Plants {
            pots: Vec::from(pots),
            rules: self.rules.clone(),
            zero_index,
        }
    }

    fn sum(&self) -> i64 {
        self.pots
            .iter()
            .enumerate()
            .filter(|(_, p)| **p == Pot::Plant)
            .map(|x| x.0 as i64 - self.zero_index as i64)
            .sum()
    }
}

impl FromStr for Plants {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();

        let pots = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .bytes()
            .map(Pot::from_u8)
            .collect();

        let rules: HashMap<Vec<Pot>, Pot> = lines
            .filter(|l| l.chars().any(|c| !c.is_whitespace()))
            .map(|l| {
                let mut columns = l.split_whitespace();
                let pattern = columns.next().unwrap().bytes().map(Pot::from_u8).collect();
                let result = columns
                    .last()
                    .unwrap()
                    .bytes()
                    .map(Pot::from_u8)
                    .last()
                    .unwrap();

                (pattern, result)
            })
            .collect();

        Ok(Plants {
            pots,
            rules,
            zero_index: 0,
        })
    }
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let plants: Plants = input.parse().unwrap();

    println!(
        "Part 1. Pots with plant in gen 20: {}",
        plants.evolve(20).sum()
    );

    let mut factor = 0;
    let mut last_two_digits = 0;
    for i in (200..=500).step_by(100) {
        let sum = plants.evolve(i).sum();
        last_two_digits = sum % 100;
        factor = (sum - last_two_digits) / i as i64;
        println!(
            "Gen {0:4} sum: {1:5} = {0:4} * {2:3} + {3:2}",
            i,
            plants.evolve(i).sum(),
            factor,
            last_two_digits
        );
    }

    println!(
        "Part 2. Guess: 50000000000 * {} + {} = {}",
        factor,
        last_two_digits,
        50_000_000_000 * factor + last_two_digits
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> String {
        "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"
            .into()
    }

    #[test]
    fn parse() {
        let plants: Plants = data().parse().expect("Failed to parse test data");

        let to_pots = |s: &str| s.bytes().map(Pot::from_u8).collect::<Vec<Pot>>();

        assert_eq!(to_pots("#..#.#..##......###...###"), plants.pots);
        assert_eq!(14, plants.rules.len());
        assert_eq!(0, plants.zero_index);

        let rule = |chars: &str| *plants.rules.get(&to_pots(chars)).expect("Missing rule");

        assert_eq!(rule("...##"), Pot::Plant);
        assert_eq!(rule("..#.."), Pot::Plant);
        assert_eq!(rule(".#..."), Pot::Plant);
        assert_eq!(rule(".#.#."), Pot::Plant);
        assert_eq!(rule(".#.##"), Pot::Plant);
        assert_eq!(rule(".##.."), Pot::Plant);
        assert_eq!(rule(".####"), Pot::Plant);
        assert_eq!(rule("#.#.#"), Pot::Plant);
        assert_eq!(rule("#.###"), Pot::Plant);
        assert_eq!(rule("##.#."), Pot::Plant);
        assert_eq!(rule("##.##"), Pot::Plant);
        assert_eq!(rule("###.."), Pot::Plant);
        assert_eq!(rule("###.#"), Pot::Plant);
        assert_eq!(rule("####."), Pot::Plant);
    }

    #[test]
    fn example_1() {
        let plants: Plants = data().parse().expect("Failed to parse test data");

        let expected_indexes = [
            -2, 3, 4, 9, 10, 11, 12, 13, 17, 18, 19, 20, 21, 22, 23, 28, 30, 33, 34,
        ]
        .to_vec();

        let p = plants.evolve(20);

        let indexes: Vec<i64> = p
            .pots
            .iter()
            .enumerate()
            .filter(|(_, p)| **p == Pot::Plant)
            .map(|x| x.0 as i64 - p.zero_index as i64)
            .collect();

        assert_eq!(expected_indexes, indexes);

        assert_eq!(325, p.sum());
    }
}

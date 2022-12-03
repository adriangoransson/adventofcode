use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Could not read stdin");

    let prepared = prepare_input(&input);
    let (part_1, part_2) = (part_1(&prepared), part_2(&prepared));

    println!("Part 1: {part_1}\nPart 2: {part_2}")
}

fn prepare_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part_1(lines: &[&str]) -> u32 {
    let errors: Vec<_> = lines
        .iter()
        .map(|line| {
            let half = line.len() / 2;

            let group1: HashSet<_> = line.chars().take(half).collect();
            let group2: HashSet<_> = line.chars().skip(half).collect();

            let error = group1.intersection(&group2).next().unwrap();
            *error
        })
        .collect();

    priorities_sum(&errors)
}

fn part_2(lines: &[&str]) -> u32 {
    let mut badges = Vec::with_capacity(lines.len() / 3);

    let mut iter = lines.iter();
    while let Some((l1, l2, l3)) = iter.next().and_then(|l1| {
        iter.next()
            .and_then(|l2| iter.next().map(|l3| (l1, l2, l3)))
    }) {
        let group1: HashSet<_> = l1.chars().collect();
        let group2: HashSet<_> = l2.chars().collect();
        let group3: HashSet<_> = l3.chars().collect();

        for c in group1 {
            if group2.contains(&c) && group3.contains(&c) {
                badges.push(c);
                break;
            }
        }
    }

    priorities_sum(&badges)
}

fn priorities_sum(chars: &[char]) -> u32 {
    chars
        .iter()
        .map(|c| {
            let c = *c as u8;

            let result = if c.is_ascii_lowercase() {
                c - b'a' + 1 // 01 through 26.
            } else {
                c - b'A' + 27 // 27 through 52.
            };

            result as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_1() {
        let sum = super::part_1(&prepare_input(EXAMPLE));
        assert_eq!(sum, 157)
    }

    #[test]
    fn part_2() {
        let sum = super::part_2(&prepare_input(EXAMPLE));
        assert_eq!(sum, 70);
    }
}

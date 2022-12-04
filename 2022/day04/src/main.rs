use std::{
    io::{stdin, Read},
    ops::RangeInclusive,
    str::Chars,
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

type Range = RangeInclusive<u32>;
type RangePair = (Range, Range);

fn prepare_input(input: &str) -> Vec<RangePair> {
    let next_int = |chars: &mut Chars| {
        let mut d = 0;

        while let Some(c) = chars.next() {
            if !c.is_digit(10) {
                break;
            }

            let value = c as u32 - '0' as u32;
            d = d * 10 + value;
        }

        d
    };

    let next_range = |chars: &mut Chars| next_int(chars)..=next_int(chars);

    input
        .lines()
        .map(str::chars)
        .map(|mut chars| (next_range(&mut chars), next_range(&mut chars)))
        .collect()
}

fn part_1(lines: &[RangePair]) -> u32 {
    lines
        .iter()
        .filter(|(r1, r2)| {
            r1.contains(r2.start()) && r1.contains(r2.end())
                || r2.contains(r1.start()) && r2.contains(r1.end())
        })
        .count() as u32
}

fn part_2(lines: &[RangePair]) -> u32 {
    lines
        .iter()
        .filter(|(r1, r2)| {
            r1.contains(r2.start())
                || r1.contains(r2.end())
                || r2.contains(r1.start())
                || r2.contains(r1.end())
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1() {
        let answer = super::part_1(&prepare_input(EXAMPLE));
        assert_eq!(answer, 2);
    }

    #[test]
    fn part_2() {
        let answer = super::part_2(&prepare_input(EXAMPLE));
        assert_eq!(answer, 4);
    }
}

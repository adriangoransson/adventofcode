use std::{
    collections::{HashSet, VecDeque},
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

type Prepared = str;
type Output = usize;

fn prepare_input(input: &str) -> &Prepared {
    input
}

fn part_1(p: &Prepared) -> Output {
    find_unique(p, 4).unwrap()
}

fn part_2(p: &Prepared) -> Output {
    find_unique(p, 14).unwrap()
}

fn find_unique(p: &Prepared, length: usize) -> Option<Output> {
    let mut last = p.chars().take(length).collect::<VecDeque<_>>();

    for (i, c) in p.chars().enumerate().skip(length) {
        if last.iter().collect::<HashSet<_>>().len() == length {
            return Some(i);
        }

        last.pop_front().unwrap();
        last.push_back(c);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [(&str, Output, Output); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn part_1() {
        for (i, (input, expected, _)) in EXAMPLE.into_iter().enumerate() {
            let answer = super::part_1(&prepare_input(input));
            assert_eq!(answer, expected, "Test case {i}");
        }
    }

    #[test]
    fn part_2() {
        for (i, (input, _, expected)) in EXAMPLE.into_iter().enumerate() {
            let answer = super::part_2(&prepare_input(input));
            assert_eq!(answer, expected, "Test case {i}");
        }
    }
}

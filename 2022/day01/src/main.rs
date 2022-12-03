use std::io::{stdin, Read};

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
    totals_by_elf(lines).into_iter().max().unwrap_or(0)
}

fn part_2(lines: &[&str]) -> u32 {
    let mut totals = totals_by_elf(lines);
    totals.sort();

    totals.iter().rev().take(3).sum()
}

fn totals_by_elf(lines: &[&str]) -> Vec<u32> {
    let mut totals = Vec::new();

    let mut current = 0;
    for line in lines {
        if line.is_empty() {
            totals.push(current);
            current = 0;
            continue;
        }

        current += line.parse::<u32>().unwrap_or(0);
    }

    if current > 0 {
        totals.push(current);
    }

    totals
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_1() {
        let max = super::part_1(&prepare_input(EXAMPLE));
        assert_eq!(max, 24000)
    }

    #[test]
    fn part_2() {
        let sum = super::part_2(&prepare_input(EXAMPLE));
        assert_eq!(sum, 45000)
    }
}

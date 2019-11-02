use std::collections::HashSet;
use std::io::{stdin, Read};

fn run(input: &str) -> (i32, i32) {
    let numbers: Vec<i32> = input.split_whitespace().flat_map(str::parse).collect();

    let mut sums: HashSet<i32> = HashSet::new();
    let mut sum = 0;
    for num in numbers.iter().cycle() {
        sum += num;
        if sums.contains(&sum) {
            break;
        } else {
            sums.insert(sum);
        }
    }

    (numbers.iter().sum(), sum)
}
fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let (sum, first_duplicate) = run(&input);

    println!("Part 1. Sum: {}", sum);
    println!("Part 2. First duplicate sum: {}", first_duplicate);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> String {
        "+1
        -2
        +3
        +1"
        .into()
    }

    #[test]
    fn part1() {
        assert_eq!(3, run(&data()).0);
    }

    #[test]
    fn part2() {
        assert_eq!(2, run(&data()).1);
    }
}

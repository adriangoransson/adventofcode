use std::collections::HashSet;
use std::io::{stdin, Read};

fn remove_reactions(polymer: &[u8]) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(polymer.len());

    let mut units_reacted = false;
    let mut iter = polymer.iter();

    // Mess. Basically try to get a next byte to compare with and
    // if they are equal, check if they also react.
    let mut current = iter.next().unwrap();
    loop {
        if let Some(next) = iter.next() {
            if units_react(*current, *next) {
                // If they are equal and react, set a flag so we can do another
                // pass recusively after the loop, advance the iterator or break.
                units_reacted = true;
                match iter.next() {
                    Some(b) => current = b,
                    None => break,
                };
            } else {
                // If equal but no reaction, push current byte to return vec,
                // set current = next.
                ret.push(*current);
                current = next;
            }
        } else {
            // If no equality, push the current char, advance the iterator or break.
            ret.push(*current);
            match iter.next() {
                Some(b) => current = b,
                None => break,
            };
        }
    }

    if !units_reacted {
        ret
    } else {
        remove_reactions(&ret)
    }
}

fn units_react(u1: u8, u2: u8) -> bool {
    u1.eq_ignore_ascii_case(&u2) && u1.is_ascii_lowercase() != u2.is_ascii_lowercase()
}

fn polymer_without_unit(polymer: &[u8], unit: u8) -> Vec<u8> {
    polymer
        .iter()
        .filter(|x| !x.eq_ignore_ascii_case(&unit))
        .cloned()
        .collect()
}

fn run(input: &str) -> (usize, usize) {
    let input = input.trim().as_bytes();

    let result = remove_reactions(&input);

    // Collect all unique (case insensitive) units to remove.
    // Case doesn't matter so the resulting set size can be shrunk even further.
    let bytes: HashSet<u8> = input.iter().map(u8::to_ascii_lowercase).collect();

    let min = bytes
        .iter()
        .map(|b| remove_reactions(&polymer_without_unit(&input, *b)).len())
        .min()
        .unwrap();

    (result.len(), min)
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let (length, smallest) = run(&input);
    println!("Part 1. Length: {}", length);
    println!("Part 2. Smallest polymer: {}", smallest);
}

#[cfg(test)]
mod tests {
    use super::run;

    fn data() -> String {
        String::from("dabAcCaCBAcCcaDA")
    }

    fn remove_reactions(input: &str) -> String {
        let ret = super::remove_reactions(input.as_bytes());
        String::from_utf8(ret).unwrap()
    }

    #[test]
    fn polymer_shortening() {
        assert_eq!("dabCBAcaDA", remove_reactions(&data()));

        // The edge characters react
        assert_eq!("abCBAcaD", remove_reactions("DdabAcCaCBAcCcaDAaBbCc"));

        assert_eq!("AC", remove_reactions("AaACcC"));
    }

    #[test]
    fn part1() {
        let (length, _) = run(&data());
        assert_eq!(10, length);
    }

    #[test]
    fn part2() {
        let (_, smallest) = run(&data());
        assert_eq!(4, smallest);
    }
}

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Could not read stdin");

    let (checksum, common) = run(&input);

    println!("Part 1. Checksum: {}", checksum);
    println!("Part 2. Common {}", common.unwrap_or_else(|| "None".into()));
}

fn run(input: &str) -> (u32, Option<String>) {
    let lines: Vec<&str> = input.split_whitespace().collect();

    (part_1(&lines), part_2(&lines))
}

fn part_1(lines: &[&str]) -> u32 {
    let mut two = 0;
    let mut three = 0;
    for line in lines {
        let mut ascii_char_count = [0; 256];
        line.bytes().for_each(|b| ascii_char_count[b as usize] += 1);

        if ascii_char_count.iter().any(|c| *c == 2) {
            two += 1;
        }

        if ascii_char_count.iter().any(|c| *c == 3) {
            three += 1;
        }
    }

    two * three
}

fn part_2(lines: &[&str]) -> Option<String> {
    for (index, line_1) in lines.iter().enumerate() {
        for line_2 in lines.iter().skip(index) {
            let diff_chars = line_1
                .chars()
                .zip(line_2.chars())
                .filter(|(fst, snd)| fst != snd)
                .count();

            if diff_chars == 1 {
                let common: String = line_1
                    .chars()
                    .zip(line_2.chars())
                    .filter(|(fst, snd)| fst == snd)
                    .map(|x| x.0)
                    .collect();

                return Some(common);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data1() -> String {
        "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"
            .into()
    }

    fn data2() -> String {
        "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"
            .into()
    }

    #[test]
    fn part1() {
        let (checksum, _) = run(&data1());
        assert_eq!(12, checksum);
    }

    #[test]
    fn part2() {
        let (_, common) = run(&data2());
        assert!(common.is_some());
        assert_eq!("fgij", common.unwrap());
    }
}

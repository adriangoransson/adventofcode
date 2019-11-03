use std::io::stdin;

struct RecipeScores {
    pos_1: usize,
    pos_2: usize,
    scores: Vec<u8>,
}

impl RecipeScores {
    fn new(scores: Vec<u8>) -> Self {
        RecipeScores {
            pos_1: 0,
            pos_2: 1,
            scores,
        }
    }
}

impl Iterator for RecipeScores {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let sum = self.scores[self.pos_1] + self.scores[self.pos_2];

        let len_diff = {
            if sum > 9 {
                self.scores.push(1);
                self.scores.push(sum - 10);
                2
            } else {
                self.scores.push(sum);
                1
            }
        };

        self.pos_1 += 1 + self.scores[self.pos_1] as usize;
        self.pos_2 += 1 + self.scores[self.pos_2] as usize;

        self.pos_1 %= self.scores.len();
        self.pos_2 %= self.scores.len();

        Some(len_diff)
    }
}

fn part1(recipe_count: usize) -> String {
    let mut rc = RecipeScores::new(vec![3, 7]);

    while rc.scores.len() < recipe_count + 10 {
        rc.next();
    }

    rc.scores[recipe_count..]
        .iter()
        .take(10)
        .map(|s| (b'0' + s) as char)
        .collect()
}

fn part2(needle: &str) -> usize {
    let scores: Vec<u8> = needle
        .bytes()
        .map(|b| b - b'0') // Remove ascii point
        .collect();

    let mut rc = RecipeScores::new(vec![3, 7]);
    let mut last_seen = 0;

    loop {
        let len_diff = rc.next().unwrap();

        let start = rc.scores.len() - len_diff;
        for (i, &s) in rc.scores[start..].iter().enumerate() {
            if last_seen == scores.len() {
                return rc.scores.len() - scores.len() - i;
            }

            last_seen = {
                if s == scores[last_seen] {
                    last_seen + 1
                } else if s == scores[0] {
                    1
                } else {
                    0
                }
            };
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read stdin");

    let recipe_count = input.trim().parse().expect("Invalid number");
    println!("Part 1: {}", part1(recipe_count));

    println!("Part 2: {}", part2(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn p1_after_9() {
        assert_eq!("5158916779", part1(9));
    }

    #[test]
    fn p1_after_5() {
        assert_eq!("0124515891", part1(5))
    }

    #[test]
    fn p1_after_18() {
        assert_eq!("9251071085", part1(18));
    }

    #[test]
    fn p1_after_2018() {
        assert_eq!("5941429882", part1(2018));
    }

    #[test]
    fn p2_after_9() {
        assert_eq!(9, part2("51589"));
    }

    #[test]
    fn p2_after_5() {
        assert_eq!(5, part2("01245"))
    }

    #[test]
    fn p2_after_18() {
        assert_eq!(18, part2("92510"));
    }

    #[test]
    fn p2_after_2018() {
        assert_eq!(2018, part2("59414"));
    }
}

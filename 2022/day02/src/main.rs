use std::{
    io::{stdin, Read},
    str::FromStr,
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

fn prepare_input(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|line| {
            let mut symbols = line.split_whitespace();
            let (o, p) = symbols
                .next()
                .and_then(|o| symbols.next().map(|p| (o, p)))
                .expect("invalid input?");

            let opponent: Symbol = o.parse().expect("invalid symbol?");
            let player = p.parse().expect("invalid symbol?");

            Round {
                opponent: opponent.into(),
                player,
            }
        })
        .collect()
}

fn part_1(rounds: &[Round]) -> u32 {
    rounds
        .iter()
        .map(|r| player_score(r.opponent, r.player.into()))
        .sum()
}

fn part_2(rounds: &[Round]) -> u32 {
    rounds
        .iter()
        .map(|r| {
            let player = match r.player {
                Symbol::X => r.opponent.wins_over(),
                Symbol::Z => r.opponent.loses_to(),
                Symbol::Y => r.opponent,
                _ => unreachable!(),
            };

            player_score(r.opponent, player)
        })
        .sum()
}

struct Round {
    opponent: HandShape,
    player: Symbol,
}

fn player_score(opponent: HandShape, player: HandShape) -> u32 {
    player.outcome_against(opponent) as u32 + player as u32
}

#[derive(Clone, Copy, PartialEq)]
enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl HandShape {
    fn outcome_against(self, other: HandShape) -> Outcome {
        if self == other {
            Outcome::Draw
        } else if self.wins_over() == other {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }

    fn wins_over(&self) -> HandShape {
        match self {
            HandShape::Rock => HandShape::Scissor,
            HandShape::Paper => HandShape::Rock,
            HandShape::Scissor => HandShape::Paper,
        }
    }

    fn loses_to(&self) -> HandShape {
        match self {
            HandShape::Rock => HandShape::Paper,
            HandShape::Paper => HandShape::Scissor,
            HandShape::Scissor => HandShape::Rock,
        }
    }
}

impl From<Symbol> for HandShape {
    fn from(s: Symbol) -> Self {
        match s {
            Symbol::A => Self::Rock,
            Symbol::B => Self::Paper,
            Symbol::C => Self::Scissor,

            Symbol::X => Self::Rock,
            Symbol::Y => Self::Paper,
            Symbol::Z => Self::Scissor,
        }
    }
}

#[derive(Clone, Copy)]
enum Symbol {
    A,
    B,
    C,

    X,
    Y,
    Z,
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "X" => Self::X,
            "Y" => Self::Y,
            "Z" => Self::Z,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn part_1() {
        let score = super::part_1(&prepare_input(EXAMPLE));
        assert_eq!(score, 15)
    }

    #[test]
    fn part_2() {
        let score = super::part_2(&prepare_input(EXAMPLE));
        assert_eq!(score, 12)
    }
}

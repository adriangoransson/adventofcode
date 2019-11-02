use std::collections::VecDeque;
use std::io::{stdin, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Game {
    players: usize,
    last_marble: usize,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let players = it.next().unwrap().parse().unwrap();
        let last_marble = it.nth(5).unwrap().parse().unwrap();

        Ok(Game {
            players,
            last_marble,
        })
    }
}

fn run(game: &Game) -> u32 {
    let mut player_scores = vec![0; game.players + 1];
    let mut circle = VecDeque::with_capacity(game.last_marble);
    circle.push_front(0);

    // Player turns go in cycles but the game plays until the last marble.
    let players = (1..).take(game.players).cycle();
    let marbles = (1..).take(game.last_marble);

    // Initial, naive solution to just calculate an index in a Vec and insert
    // (rearranging the poor vec every time) didn't really work for part 2...
    // Instead use a double ended queue and rotate it so insert/remove ops are
    // always done at the front.
    for (player, marble) in players.zip(marbles) {
        if marble % 23 == 0 {
            circle.rotate_right(7); // Safe because circle.len() >= 22.
            let replaced_marble = circle.pop_front().unwrap();
            player_scores[player] += replaced_marble + marble;
        } else {
            // Try to rotate two steps. Rotation > circle.len() panics.
            circle.rotate_left(2.min(circle.len()));
            circle.push_front(marble);
        }
    }

    *player_scores.iter().max().unwrap()
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let mut game: Game = input.parse().unwrap();
    let part1 = run(&game);

    game.last_marble *= 100;
    let part2 = run(&game);

    println!("Part 1. High score: {}", part1);
    println!("Part 2. High score: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::run;

    #[rustfmt::skip]
    fn data() -> Vec<(String,u32)> {
        vec![
            (" 9 players; last marble is worth   25 points".into(), 32),
            ("10 players; last marble is worth 1618 points".into(), 8_317),
            ("13 players; last marble is worth 7999 points".into(), 146_373),
            ("17 players; last marble is worth 1104 points".into(), 2_764),
            ("21 players; last marble is worth 6111 points".into(), 54_718),
            ("30 players; last marble is worth 5807 points".into(), 37_305),
        ]
    }

    // Run tests in parallel without all the duplicate code.
    // Maybe a macro would be a better idea.
    fn test(data_index: usize) {
        let (test, expected) = &data()[data_index];
        let game = test.parse().unwrap();
        assert_eq!(*expected, run(&game));
    }

    #[test]
    fn example_1() {
        test(0);
    }

    #[test]
    fn example_2() {
        test(1);
    }

    #[test]
    fn example_3() {
        test(2);
    }

    #[test]
    fn example_5() {
        test(3);
    }

    #[test]
    fn example_6() {
        test(4);
    }

    #[test]
    fn example_7() {
        test(5);
    }
}

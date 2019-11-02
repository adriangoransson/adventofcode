use std::io::{stdin, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Claim {
    id: u32,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let id = it
            .next()
            .unwrap()
            .trim_matches('#')
            .parse()
            .expect("Invalid ID"); // id

        it.next(); // @

        let coordinates: Vec<i32> = it
            .next()
            .unwrap()
            .trim_matches(':')
            .split(',')
            .flat_map(str::parse)
            .collect();

        let (x, y) = (coordinates[0], coordinates[1]);

        let size: Vec<u32> = it.next().unwrap().split('x').flat_map(str::parse).collect();
        let (width, height) = (size[0], size[1]);

        Ok(Claim {
            id,
            x,
            y,
            width,
            height,
        })
    }
}

fn run(input: &str) -> (u32, u32) {
    let claims: Vec<Claim> = input.lines().flat_map(str::parse).collect();

    let mut max_x = 0;
    let mut max_y = 0;
    for c in &claims {
        max_x = max_x.max(c.x + c.width as i32);
        max_y = max_y.max(c.y + c.height as i32);
    }

    let mut coordinate_claims = vec![vec![0; max_x as usize]; max_y as usize];

    for claim in &claims {
        for y in (claim.y..).take(claim.height as usize) {
            for x in (claim.x..).take(claim.width as usize) {
                coordinate_claims[y as usize][x as usize] += 1;
            }
        }
    }

    let overlapping = coordinate_claims
        .iter()
        .map(|row| row.iter().filter(|count| **count > 1).count())
        .sum::<usize>() as u32;

    let lonely = claims
        .iter()
        .find(|claim| {
            let contested = (claim.y..).take(claim.height as usize).any(|row| {
                (claim.x..)
                    .take(claim.width as usize)
                    .any(|column| coordinate_claims[row as usize][column as usize] > 1)
            });

            !contested
        })
        .expect("No lone claim found");

    (overlapping, lonely.id)
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let (overlapping, id) = run(&input);

    println!("Overlapping squares: {}", overlapping);
    println!("Single area without overlap: #{}", id);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> String {
        "#1 @ 1,3: 4x4
    #2 @ 3,1: 4x4
    #3 @ 5,5: 2x2"
            .into()
    }

    #[test]
    fn part1() {
        let (overlapping, _) = run(&data());
        assert_eq!(4, overlapping);
    }

    #[test]
    fn part2() {
        let (_, lone_id) = run(&data());
        assert_eq!(3, lone_id);
    }
}

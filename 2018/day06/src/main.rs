use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

#[derive(Clone, Debug)]
enum Point {
    Empty,
    Coordinate { x: i32, y: i32 },
    ClosestTo { x: i32, y: i32, distance: u32 },
    ClosestToMany { distance: u32 },
}

fn manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u32 {
    let distance = (x2 - x1).abs() + (y2 - y1).abs();
    distance as u32
}

fn get_point(x: i32, y: i32, coordinates: &HashSet<(i32, i32)>) -> Point {
    coordinates
        .iter()
        .zip(coordinates.iter().map(|&c| manhattan_distance(c, (x, y))))
        .fold(
            Point::Empty,
            |acc, (&(cur_x, cur_y), cur_distance)| match acc {
                Point::Empty => Point::ClosestTo {
                    x: cur_x,
                    y: cur_y,
                    distance: cur_distance,
                },

                Point::ClosestTo { x, y, distance } => match cur_distance.cmp(&distance) {
                    Ordering::Equal => Point::ClosestToMany {
                        distance: cur_distance,
                    },
                    Ordering::Less => Point::ClosestTo {
                        x: cur_x,
                        y: cur_y,
                        distance: cur_distance,
                    },
                    Ordering::Greater => Point::ClosestTo { x, y, distance },
                },

                Point::ClosestToMany { distance } => match cur_distance.cmp(&distance) {
                    Ordering::Equal => Point::ClosestToMany {
                        distance: cur_distance,
                    },
                    Ordering::Less => Point::ClosestTo {
                        x: cur_x,
                        y: cur_y,
                        distance: cur_distance,
                    },
                    Ordering::Greater => Point::ClosestToMany { distance },
                },

                point => point,
            },
        )
}

fn run(input: &str, boundary: u32) -> (i32, u32) {
    let coordinates: HashSet<(i32, i32)> = input
        .lines()
        .map(|line| {
            let mut co = line.split(", ");
            let x = co.next().unwrap().parse().unwrap();
            let y = co.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    let mut max_x = 0;
    let mut max_y = 0;
    coordinates.iter().for_each(|&(x, y)| {
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    });

    let mut grid = vec![vec![Point::Empty; (max_x + 1) as usize]; (max_y + 1) as usize];

    // enumerate() yields index as usize, zip to avoid recasting
    for (y, row) in (0..).zip(grid.iter_mut()) {
        for (x, column) in (0..).zip(row.iter_mut()) {
            if coordinates.contains(&(x, y)) {
                *column = Point::Coordinate { x, y };
            } else {
                *column = get_point(x, y, &coordinates);
            }
        }
    }

    let mut areas: HashMap<(i32, i32), Option<i32>> =
        coordinates.iter().map(|&c| (c, Some(0))).collect();

    for (y, row) in grid.iter().enumerate() {
        for (x, column) in row.iter().enumerate() {
            let coordinate = match *column {
                Point::Coordinate { x, y } | Point::ClosestTo { x, y, .. } => (x, y),
                _ => continue,
            };

            let entry = areas.get_mut(&coordinate).unwrap();

            if let Some(count) = entry {
                // Edge of grid == infinite area
                if x == 0 || y == 0 || x == max_x as usize || y == max_y as usize {
                    *entry = None;
                } else {
                    *count += 1;
                }
            }
        }
    }

    let largest = areas.values().flatten().max().unwrap();

    let mut area = 0;
    for x in 0..=max_x {
        for y in 0..=max_y {
            let sum: u32 = coordinates
                .iter()
                .map(|&c| manhattan_distance(c, (x, y)))
                .sum();

            if sum < boundary {
                area += 1;
            }
        }
    }

    (*largest, area)
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let (largest, area) = run(&input, 10000);
    println!("Part 1. Largest area: {}", largest);
    println!("Part 2. Largest area: {}", area);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> String {
        String::from(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
        )
    }

    #[test]
    fn test_part1() {
        let (largest, _) = run(&data(), 32);
        assert_eq!(17, largest);
    }

    #[test]
    fn test_part2() {
        let (_, area) = run(&data(), 32);
        assert_eq!(16, area);
    }
}

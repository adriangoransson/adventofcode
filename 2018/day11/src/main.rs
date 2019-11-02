use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{stdin, Read};

const GRID_SIDE: usize = 300;

type Grid = Vec<Vec<i32>>;
type Serial = i32;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn from_zero_index(x: usize, y: usize) -> Self {
        Coordinate {
            x: (x + 1) as i32,
            y: (y + 1) as i32,
        }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq)]
struct Square {
    top_left: Coordinate,
    size: u32,
    sum: i32,
}

fn power_level(coord: Coordinate, serial: Serial) -> i32 {
    let rack_id = coord.x + 10;
    let mut power = rack_id * coord.y;
    power += serial;
    power *= rack_id;

    power /= 100;
    power %= 10;

    power - 5
}

struct PowerGrid(Grid);

impl PowerGrid {
    fn new(serial: Serial) -> Self {
        let mut grid = vec![vec![0; GRID_SIDE]; GRID_SIDE];

        for (y, row) in grid.iter_mut().enumerate() {
            for (x, column) in row.iter_mut().enumerate() {
                let coord = Coordinate::from_zero_index(x, y);

                *column = power_level(coord, serial);
            }
        }

        PowerGrid(grid)
    }

    fn largest_square_by_size(&self, size: usize) -> Square {
        let mut largest_sum = std::i32::MIN;
        let mut top_left = Coordinate::default();

        let limit = GRID_SIDE - size;

        for y in 0..limit {
            for x in 0..limit {
                let mut sum = 0;

                for y in (y..).take(size) {
                    for x in (x..).take(size) {
                        sum += self.0[y][x];
                    }
                }

                if sum > largest_sum {
                    largest_sum = sum;
                    top_left = Coordinate::from_zero_index(x, y);
                }
            }
        }

        Square {
            top_left,
            sum: largest_sum,
            size: size as u32,
        }
    }

    fn sum_area_table(&self) -> SumAreaTable {
        let mut table = vec![vec![0; GRID_SIDE]; GRID_SIDE];
        for y in 0..GRID_SIDE {
            for x in 0..GRID_SIDE {
                table[y][x] = {
                    let above = if y > 0 { table[y - 1][x] } else { 0 };
                    let left = if x > 0 { table[y][x - 1] } else { 0 };

                    let prev = if y > 0 && x > 0 {
                        table[y - 1][x - 1]
                    } else {
                        0
                    };

                    self.0[y][x] + above + left - prev
                };
            }
        }

        SumAreaTable(table)
    }
}

struct SumAreaTable(Grid);

impl SumAreaTable {
    fn largest_square_by_size(&self, size: usize) -> Square {
        let table = &self.0;
        let limit = GRID_SIDE - size;

        let mut largest_sum = std::i32::MIN;
        let mut top_left = Coordinate::default();

        for top in 0..limit {
            for left in 0..limit {
                let bottom = top + size;
                let right = left + size;

                let sum = table[top][left] - table[top][right] - table[bottom][left]
                    + table[bottom][right];

                if sum > largest_sum {
                    largest_sum = sum;
                    top_left = Coordinate::from_zero_index(left + 1, top + 1);
                }
            }
        }

        Square {
            top_left,
            sum: largest_sum,
            size: size as u32,
        }
    }

    fn find_largest_square(&self) -> Square {
        let mut largest_square = Square::default();

        for size in (1..).take(GRID_SIDE) {
            let sq = self.largest_square_by_size(size);
            if sq.sum > largest_square.sum {
                largest_square = sq;
            }
        }

        largest_square
    }
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let serial = input.parse().expect("Serial is not a valid number");
    let grid = PowerGrid::new(serial);

    // I learned about summed area tables from the subreddit. A much faster
    // solution for part 2 than to calculate the sum for every subgrid.
    // (https://en.wikipedia.org/wiki/Summed-area_table)
    let table = grid.sum_area_table();

    // Brute force is a little faster for small grids.
    let part1 = grid.largest_square_by_size(3);

    // But for part 2 where there are many more, and varied grids:
    // Use a sum area table for part 2.
    let part2 = table.find_largest_square();

    println!(
        "Part 1. Largest 3x3 square's top left coordinate is {}. Total power {}.",
        part1.top_left, part1.sum
    );

    println!(
        "Part 2. Largest {0}x{0} square's top left coordinate is {1}. Total power {2}.",
        part2.size, part2.top_left, part2.sum
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_level_1() {
        assert_eq!(
            4,
            power_level(Coordinate { x: 3, y: 5 }, 8),
            "Fuel cell at 3,5 in a grid with serial number 8: power level 4."
        );
    }

    #[test]
    fn power_level_2() {
        assert_eq!(
            -5,
            power_level(Coordinate { x: 122, y: 79 }, 57),
            "Fuel cell at  122,79, grid serial number 57: power level -5."
        );
    }

    #[test]
    fn power_level_3() {
        assert_eq!(
            0,
            power_level(Coordinate { x: 217, y: 196 }, 39),
            "Fuel cell at 217,196, grid serial number 39: power level  0."
        );
    }

    #[test]
    fn power_level_4() {
        assert_eq!(
            4,
            power_level(Coordinate { x: 101, y: 153 }, 71),
            "Fuel cell at 101,153, grid serial number 71: power level  4."
        );
    }

    #[test]
    fn example_1() {
        let expected = Square {
            top_left: Coordinate { x: 33, y: 45 },
            sum: 29,
            size: 3,
        };

        let grid = PowerGrid::new(18);
        let square1 = grid.largest_square_by_size(3);

        assert_eq!(expected, square1);

        let table = grid.sum_area_table();
        let square2 = table.largest_square_by_size(3);

        assert_eq!(
            square1, square2,
            "Largest square from sum table did not match largest square from power grid"
        );
    }

    #[test]
    fn example_2() {
        let expected = Square {
            top_left: Coordinate { x: 21, y: 61 },
            sum: 30,
            size: 3,
        };

        let grid = PowerGrid::new(42);
        let square1 = grid.largest_square_by_size(3);

        assert_eq!(expected, square1);

        let table = grid.sum_area_table();
        let square2 = table.largest_square_by_size(3);

        assert_eq!(
            square1, square2,
            "Largest square from sum table did not match largest square from power grid"
        );
    }
}

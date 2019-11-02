use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{stdin, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point {
    position: Coordinate,
    velocity: Coordinate,
}

impl Point {
    fn transform(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
}

impl FromStr for Point {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let terminators: &[char] = &['<', '>', ','];
        let fields = input.split_terminator(terminators).collect::<Vec<&str>>();

        assert_eq!(6, fields.len(), "Invalid input {}", input);

        let pos_x = fields[1].trim().parse().unwrap();
        let pos_y = fields[2].trim().parse().unwrap();

        let vel_x = fields[4].trim().parse().unwrap();
        let vel_y = fields[5].trim().parse().unwrap();

        Ok(Point {
            position: Coordinate { x: pos_x, y: pos_y },
            velocity: Coordinate { x: vel_x, y: vel_y },
        })
    }
}

struct Points(Vec<Point>);

impl Points {
    fn transform_all(&mut self) {
        self.0.iter_mut().for_each(Point::transform);
    }

    fn dimensions(&self) -> Dimensions {
        let points = &self.0;

        let mut min_x: i32 = std::i32::MAX;
        let mut min_y: i32 = std::i32::MAX;
        let mut max_x: i32 = std::i32::MIN;
        let mut max_y: i32 = std::i32::MIN;

        for &Point {
            position: Coordinate { x, y },
            ..
        } in points
        {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        let width = (max_x - min_x) as u32;
        let height = (max_y - min_y) as u32;
        let area = width * height;

        Dimensions {
            min_x,
            min_y,
            width,
            height,
            area,
        }
    }

    fn grid(&self) -> Grid {
        let Dimensions {
            min_x,
            min_y,
            width,
            height,
            area,
            ..
        } = self.dimensions();

        assert!(area < 100_000, "Area must be less than 100 000");

        let mut grid = vec![vec!['.'; width as usize + 1]; height as usize + 1];

        for point in &self.0 {
            let column = point.position.x - min_x;
            let row = point.position.y - min_y;

            grid[row as usize][column as usize] = '#';
        }

        Grid(grid)
    }
}

struct Grid(Vec<Vec<char>>);

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let grid = &self.0;

        for row in grid {
            for column in row {
                write!(f, "{}", column)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

struct Dimensions {
    min_x: i32,
    min_y: i32,
    width: u32,
    height: u32,
    area: u32,
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let mut points = Points(input.lines().flat_map(str::parse).collect());

    // Fast forward until area is small enough for memory and visual output.
    let mut seconds = 0;
    while points.dimensions().area > 2000 {
        points.transform_all();
        seconds += 1;
    }

    let mut area = points.dimensions().area;
    loop {
        let grid = points.grid();
        points.transform_all();

        let a = points.dimensions().area;
        if a > area {
            // Grid is growing, break.
            println!("{}", &grid);
            break;
        }

        area = a;
        seconds += 1;
    }

    println!("Message would have arrived in {} seconds.", seconds);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> String {
        "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"
            .into()
    }

    fn points() -> Points {
        Points(data().lines().flat_map(str::parse).collect())
    }

    fn transform_points_n(points: &mut Points, n: usize) {
        (0..).take(n).for_each(|_| points.transform_all());
    }

    #[test]
    fn parse() {
        let points: Vec<Point> = data().lines().flat_map(str::parse).collect();

        assert_eq!(data().lines().count(), points.len());

        assert_eq!(9, points[0].position.x);
        assert_eq!(1, points[0].position.y);
        assert_eq!(0, points[0].velocity.x);
        assert_eq!(2, points[0].velocity.y);

        assert_eq!(7, points[1].position.x);
        assert_eq!(0, points[1].position.y);
        assert_eq!(-1, points[1].velocity.x);
        assert_eq!(0, points[1].velocity.y);

        assert_eq!(-3, points[30].position.x);
        assert_eq!(6, points[30].position.y);
        assert_eq!(2, points[30].velocity.x);
        assert_eq!(-1, points[30].velocity.y);
    }

    #[test]
    fn grid_0() {
        let points = points();

        let grid = points.grid();
        assert_eq!(
            "........#.............
................#.....
.........#.#..#.......
......................
#..........#.#.......#
...............#......
....#.................
..#.#....#............
.......#..............
......#...............
...#...#.#...#........
....#..#..#.........#.
.......#..............
...........#..#.......
#...........#.........
...#.......#..........
",
            &grid.to_string()
        );
    }

    #[test]
    fn grid_1() {
        let mut points = points();
        points.transform_all();
        let grid = points.grid();

        assert_eq!(
            "........#....#....
......#.....#.....
#.........#......#
..................
....#.............
..##.........#....
....#.#...........
...##.##..#.......
......#.#.........
......#...#.....#.
#...........#.....
..#.....#.#.......
",
            &grid.to_string()
        );
    }

    #[test]
    fn grid_2() {
        let mut points = points();
        transform_points_n(&mut points, 2);
        let grid = points.grid();

        assert_eq!(
            "..........#...
#..#...####..#
..............
....#....#....
..#.#.........
...#...#......
...#..#..#.#..
#....#.#......
.#...#...##.#.
....#.........
",
            &grid.to_string()
        );
    }

    #[test]
    fn grid_3() {
        let mut points = points();
        transform_points_n(&mut points, 3);
        let grid = points.grid();

        assert_eq!(
            "#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
",
            &grid.to_string()
        );
    }

    #[test]
    fn grid_4() {
        let mut points = points();
        transform_points_n(&mut points, 4);
        let grid = points.grid();

        assert_eq!(
            "........#....
....##...#.#.
..#.....#..#.
.#..##.##.#..
...##.#....#.
.......#....#
..........#..
#......#...#.
.#.....##....
...........#.
...........#.
",
            &grid.to_string()
        );
    }
}

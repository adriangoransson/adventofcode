use std::collections::{btree_map::Entry, BTreeMap, HashMap};
use std::io::{stdin, Read};

type TrackSystem = HashMap<Coordinate, Track>;
type Carts = BTreeMap<Coordinate, Cart>;

#[derive(Debug, Eq, PartialEq)]
struct Cart {
    direction: Direction,
    intersections: u8,
}

impl Cart {
    fn new(direction: Direction) -> Self {
        Cart {
            direction,
            intersections: 0,
        }
    }

    fn next_position(&self, Coordinate { x, y }: Coordinate) -> Coordinate {
        use Direction::*;

        let new_pos = match self.direction {
            Up => (x, y - 1),
            Down => (x, y + 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        };

        new_pos.into()
    }

    fn turn(&mut self, track: Track) {
        use Direction::*;

        let curve_up = || match self.direction {
            Up => Right,
            Down => Left,
            Left => Down,
            Right => Up,
        };

        let curve_down = || match self.direction {
            Up => Left,
            Down => Right,
            Left => Up,
            Right => Down,
        };

        let turn_right = || match self.direction {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        };

        let turn_left = || match self.direction {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        };

        let dir = match track {
            Track::Intersection => {
                let dir = match self.intersections {
                    0 => turn_left(),
                    1 => self.direction,
                    2 => turn_right(),
                    _ => unreachable!(),
                };

                self.intersections = (self.intersections + 1) % 3;

                dir
            }
            Track::CurveUp => curve_up(),
            Track::CurveDown => curve_down(),
            _ => unreachable!(),
        };

        self.direction = dir;
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coordinate {
    y: usize, // Order by y then x
    x: usize,
}

impl From<(usize, usize)> for Coordinate {
    fn from((x, y): (usize, usize)) -> Self {
        Coordinate { x, y }
    }
}

#[derive(Eq, Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, Clone, Copy, Debug, PartialEq)]
enum Track {
    Vertical,
    Horizontal,
    Intersection,
    CurveUp,   // / approached from left
    CurveDown, // \ approached from left
}

fn parse_puzzle(input: &str) -> (TrackSystem, Carts) {
    let mut map = HashMap::new();
    let mut carts = BTreeMap::new();

    for (y, row) in input.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            use Direction::*;
            use Track::*;

            let coordinate = Coordinate { x, y };
            let track = match ch {
                '|' => Vertical,
                '-' => Horizontal,
                '+' => Intersection,
                '/' => CurveUp,
                '\\' => CurveDown,
                ' ' => continue,

                other => {
                    let (direction, track) = match other {
                        '^' => (Up, Vertical),
                        'v' => (Down, Vertical),
                        '<' => (Left, Horizontal),
                        '>' => (Right, Horizontal),
                        _ => unreachable!("Invalid character sequence."),
                    };

                    carts.insert(coordinate, Cart::new(direction));
                    track
                }
            };

            map.insert(coordinate, track);
        }
    }

    (map, carts)
}

fn progress(
    tracks: &TrackSystem,
    carts: &mut Carts,
    remove_on_collision: bool,
) -> Option<Coordinate> {
    let changes: Vec<(Coordinate, Coordinate)> = carts
        .iter()
        .map(|(&coordinate, cart)| (coordinate, cart.next_position(coordinate)))
        .collect();

    for (old, next) in changes {
        let mut cart = match carts.remove(&old) {
            Some(c) => c,
            None => continue, // Only happens in part 2 if it was removed as part of a collision
        };

        let track = tracks.get(&next).unwrap();
        match track {
            Track::Intersection | Track::CurveUp | Track::CurveDown => cart.turn(*track),
            _ => (),
        };

        match carts.entry(next) {
            Entry::Occupied(e) => {
                if remove_on_collision {
                    e.remove();
                } else {
                    return Some(next);
                }
            }
            Entry::Vacant(e) => {
                e.insert(cart);
            }
        };
    }

    if remove_on_collision && carts.len() == 1 {
        carts.keys().copied().nth(0) // why is there no iter().first()??
    } else {
        None
    }
}

fn run(input: &str, is_part2: bool) -> Coordinate {
    let (tracks, mut carts) = parse_puzzle(&input);

    loop {
        if let Some(collision) = progress(&tracks, &mut carts, is_part2) {
            break collision;
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let Coordinate { x, y } = run(&input, false);
    println!("Part 1: x,y = {},{}", x, y);

    let Coordinate { x, y } = run(&input, true);
    println!("Part 2: x,y = {},{}", x, y);
}

#[cfg(test)]
mod tests {
    use super::{parse_puzzle, run, Cart, Carts, Coordinate, Direction, Track, TrackSystem};

    fn display(tracks: &TrackSystem, carts: &Carts) -> String {
        let (width, height) = {
            let mut w = 0;
            let mut h = 0;

            for k in tracks.keys() {
                w = w.max(k.x);
                h = h.max(k.y);
            }

            (w, h)
        };

        let mut out = String::new();
        for y in 0..=height {
            for x in 0..=width {
                let ch = if let Some(cart) = carts.get(&(x, y).into()) {
                    use Direction::*;

                    match cart.direction {
                        Up => '^',
                        Down => 'v',
                        Left => '<',
                        Right => '>',
                    }
                } else if let Some(t) = tracks.get(&(x, y).into()) {
                    use Track::*;

                    match t {
                        Vertical => '|',
                        Horizontal => '-',
                        Intersection => '+',
                        CurveUp => '/',
                        CurveDown => '\\',
                    }
                } else {
                    ' '
                };

                out.push(ch);
            }
            out.push('\n');
        }

        out
    }

    fn data1() -> String {
        r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "
            .into()
    }

    fn data2() -> String {
        r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"
            .into()
    }

    #[test]
    fn parse() {
        let (system, carts) = parse_puzzle(&data1());
        use Direction::*;
        use Track::*;

        assert_eq!(Some(&CurveUp), system.get(&(0, 0).into()));
        assert_eq!(Some(&Horizontal), system.get(&(2, 0).into()));
        assert_eq!(Some(&CurveDown), system.get(&(4, 0).into()));

        assert_eq!(Some(&Vertical), system.get(&(0, 1).into()));
        assert_eq!(None, system.get(&(1, 1).into()));

        assert_eq!(Some(&Intersection), system.get(&(4, 2).into()));

        assert_eq!(
            Some(&Cart {
                direction: Right,
                intersections: 0
            }),
            carts.get(&(2, 0).into())
        );

        assert_eq!(
            Some(&Cart {
                direction: Down,
                intersections: 0
            }),
            carts.get(&(9, 3).into())
        );

        let expected = [
            r"/->-\        ",
            r"|   |  /----\",
            r"| /-+--+-\  |",
            r"| | |  | v  |",
            r"\-+-/  \-+--/",
            r"  \------/   ",
            "", // final newline is always inserted
        ]
        .join("\n");

        assert_eq!(expected, display(&system, &carts));
    }

    #[test]
    fn example1() {
        assert_eq!(Coordinate { x: 7, y: 3 }, run(&data1(), false));
    }

    #[test]
    fn example2() {
        assert_eq!(Coordinate { x: 6, y: 4 }, run(&data2(), true))
    }
}

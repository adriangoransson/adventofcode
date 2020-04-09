use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    fmt::Display,
    io::{self, Read},
};

type Point = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum CreatureKind {
    Elf,
    Goblin,
}

#[derive(Copy, Eq, Clone, Debug, PartialEq)]
struct Creature {
    kind: CreatureKind,
    attack_power: u32,
    hit_points: u32,
}

#[derive(Debug)]
struct Instructions {
    move_to: Option<Point>,
    attack: Option<Point>,
}

impl Creature {
    fn elf() -> Self {
        Self::elf_with_attack_power(3)
    }

    fn goblin() -> Self {
        Creature {
            kind: CreatureKind::Goblin,
            attack_power: 3,
            hit_points: 200,
        }
    }

    fn elf_with_attack_power(attack_power: u32) -> Self {
        Creature {
            kind: CreatureKind::Elf,
            attack_power,
            hit_points: 200,
        }
    }

    fn is_enemy(&self, other: &Creature) -> bool {
        self.kind != other.kind
    }

    /// Breadth first search from initial position `(x, y)`.
    fn find_next_tile(&self, board: &GameBoard, x: usize, y: usize) -> Instructions {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        let initial_pos = (x, y);

        visited.insert(initial_pos, initial_pos); // Visited edges and their parents.

        board.adjacent_points(x, y).iter().for_each(|&adjacent| {
            visited.insert(adjacent, initial_pos);
            queue.push_back(adjacent);
        });

        while !queue.is_empty() {
            let v = queue.pop_front().unwrap();

            if let Some(tile) = board.tile(v.0, v.1) {
                match tile {
                    Tile::Creature(creature) if self.is_enemy(creature) => {
                        let (prev_x, prev_y) = visited[&v];
                        let weakest_enemy = board
                            .adjacent_points(prev_x, prev_y)
                            .iter()
                            .filter_map(|&point| match board.tile(point.0, point.1) {
                                Some(Tile::Creature(other)) if self.is_enemy(other) => {
                                    Some((point, other.hit_points))
                                }
                                _ => None,
                            })
                            .min_by_key(|i| i.1)
                            .map(|(pos, _)| pos)
                            .unwrap();

                        let mut next = v;
                        let mut distance = 1;
                        while visited[&next] != initial_pos {
                            distance += 1;
                            next = visited[&next];
                        }

                        let (move_to, attack) = match distance {
                            1 => (None, Some(weakest_enemy)),
                            2 => (Some(next), Some(weakest_enemy)),
                            _ => (Some(next), None),
                        };

                        return Instructions { move_to, attack };
                    }

                    Tile::Open => {
                        for pos in board.adjacent_points(v.0, v.1) {
                            if let Entry::Vacant(entry) = visited.entry(pos) {
                                queue.push_back(pos);
                                entry.insert(v);
                            }
                        }
                    }

                    Tile::Wall | Tile::Creature(_) => continue,
                }
            }
        }

        Instructions {
            move_to: None,
            attack: None,
        }
    }
}

#[derive(Eq, Debug, PartialEq)]
enum Tile {
    Open,
    Wall,
    Creature(Creature),
}

#[derive(Debug)]
struct GameBoard {
    tiles: Vec<Tile>,
    width: usize,
    creature_count: HashMap<CreatureKind, u32>,
    rounds: u32,
}

impl GameBoard {
    fn grid_coordinates(&self, index: usize) -> Point {
        (index % self.width + 1, index / self.width + 1)
    }

    fn grid_index(&self, x: usize, y: usize) -> usize {
        assert!(x >= 1);
        assert!(y >= 1);

        (x - 1) + (y - 1) * self.width
    }

    fn remaining_hit_points(&self) -> HashMap<CreatureKind, u32> {
        let mut hit_points = HashMap::with_capacity(self.creature_count.len());

        self.tiles.iter().for_each(|tile| match tile {
            Tile::Open | Tile::Wall => (),
            Tile::Creature(c) => {
                *hit_points.entry(c.kind).or_insert(0) += c.hit_points;
            }
        });

        hit_points
    }

    fn tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(self.grid_index(x, y))
    }

    fn adjacent_points(&self, x: usize, y: usize) -> Vec<Point> {
        let top = if y > 1 { Some((x, y - 1)) } else { None };
        let left = if x > 1 { Some((x - 1, y)) } else { None };
        let right = Some((x + 1, y));
        let down = Some((x, y + 1));

        [top, left, right, down].iter().flatten().copied().collect()
    }

    fn game_over(&self) -> bool {
        !self.creature_count.values().all(|&x| x > 0)
    }

    fn turn(&mut self) {
        if self.game_over() {
            panic!("Game over! No more rounds!")
        }

        let mut visited: HashSet<usize> = HashSet::new();

        for i in 0..self.tiles.len() {
            let mut should_be_open = Vec::new(); // Clean up queue.

            if visited.contains(&i) {
                continue;
            }

            match &self.tiles[i] {
                Tile::Open | Tile::Wall => continue,

                Tile::Creature(c) => {
                    // First creature to not have an enemy means the game is over.
                    if self.game_over() {
                        return;
                    }

                    let (x0, y0) = self.grid_coordinates(i);
                    let instructions = c.find_next_tile(&self, x0, y0);

                    let attack_power = c.attack_power; // Necessary copy.

                    if let Some((x, y)) = instructions.move_to {
                        let next = self.grid_index(x, y);
                        visited.insert(next);
                        self.tiles.swap(i, next);
                    }

                    if let Some((x, y)) = instructions.attack {
                        let index = self.grid_index(x, y);
                        if let Some(Tile::Creature(other)) = self.tiles.get_mut(index) {
                            other.hit_points = other.hit_points.saturating_sub(attack_power);

                            if other.hit_points == 0 {
                                should_be_open.push(index);
                                *self.creature_count.get_mut(&other.kind).unwrap() -= 1;
                            }
                        }
                    }
                }
            }

            visited.insert(i);
            should_be_open
                .iter()
                .for_each(|i| self.tiles[*i] = Tile::Open);
        }

        self.rounds += 1;
    }

    fn parse(elf: Creature, goblin: Creature, input: &str) -> Self {
        let mut tiles = Vec::with_capacity(input.len());
        let mut creature_count = HashMap::new();

        let mut creature = |creature: Creature| {
            *creature_count.entry(creature.kind).or_insert(0) += 1;

            Tile::Creature(creature)
        };

        let width = input.find('\n').expect("No line breaks in input");

        for ch in input.chars() {
            let t = match ch {
                '.' => Tile::Open,
                '#' => Tile::Wall,
                'E' => creature(elf),
                'G' => creature(goblin),

                '\n' => continue,

                x => unreachable!("Unexpected character [{}]", x),
            };

            tiles.push(t);
        }

        tiles.shrink_to_fit();

        GameBoard {
            tiles,
            width,
            creature_count,
            rounds: 0,
        }
    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pos = 0;

        let mut addendum = String::new();
        for tile in &self.tiles {
            if pos == self.width {
                writeln!(f, "{}", addendum)?;
                addendum.clear();
                pos = 0;
            }

            write!(
                f,
                "{}",
                match tile {
                    Tile::Wall => '#',
                    Tile::Open => '.',
                    Tile::Creature(Creature {
                        kind, hit_points, ..
                    }) => match (hit_points, kind) {
                        (0, _) => '.',
                        (_, CreatureKind::Elf) => {
                            addendum.push_str(&format!(" E({})", hit_points));
                            'E'
                        }
                        (_, CreatureKind::Goblin) => {
                            addendum.push_str(&format!(" G({})", hit_points));
                            'G'
                        }
                    },
                }
            )?;

            pos += 1;
        }

        Ok(())
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin.");

    let mut board = GameBoard::parse(Creature::elf(), Creature::goblin(), &input);

    while !board.game_over() {
        board.turn();
    }

    let remaining_hp = board.remaining_hit_points().values().sum::<u32>();

    println!(
        "Part 1. {} rounds * {} remaining hit points = {}.",
        board.rounds,
        remaining_hp,
        board.rounds * remaining_hp
    );

    let try_board = |attack_power: u32| {
        let mut board = GameBoard::parse(
            Creature::elf_with_attack_power(attack_power),
            Creature::goblin(),
            &input,
        );

        let elf_count = |board: &GameBoard| board.creature_count[&CreatureKind::Elf];
        let initial_count = elf_count(&board);

        while !board.game_over() {
            board.turn();

            if elf_count(&board) != initial_count {
                return None;
            }
        }

        Some(board)
    };

    // Try a little bisection instead of bruteforce.
    let mut a: u32 = 4;
    let mut b: u32 = 100;

    while try_board(b).is_none() {
        b += 25;
    }

    loop {
        let c = (a + b) / 2;

        if let Some(board) = try_board(c) {
            if try_board(c - 1).is_none() {
                let remaining_hp = board.remaining_hit_points().values().sum::<u32>();

                println!(
                    "Part 2. Attack power: {}. {} rounds * {} remaining hit points = {}.",
                    c,
                    board.rounds,
                    remaining_hp,
                    board.rounds * remaining_hp
                );
                break;
            }

            b = c;
        } else {
            a = c;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "\
#######
#.G.E.#
#E.G.E#
#.G.E.#
#######";

    #[test]
    fn parse_board() {
        let board = GameBoard::parse(Creature::elf(), Creature::goblin(), EXAMPLE_DATA);
        assert_eq!(4, board.creature_count[&CreatureKind::Elf]);
        assert_eq!(3, board.creature_count[&CreatureKind::Goblin]);

        assert_eq!(7, board.width);

        assert_eq!(Some(&Tile::Wall), board.tile(1, 1));

        assert_eq!(Some(&Tile::Open), board.tile(2, 2));
        assert_eq!(Some(&Tile::Creature(Creature::goblin())), board.tile(3, 2));
        assert_eq!(Some(&Tile::Open), board.tile(4, 2));

        assert_eq!(Some(&Tile::Open), board.tile(5, 3));
        assert_eq!(Some(&Tile::Creature(Creature::elf())), board.tile(5, 4));
        assert_eq!(Some(&Tile::Open), board.tile(6, 4));
        assert_eq!(Some(&Tile::Wall), board.tile(7, 5));
    }

    #[test]
    fn convert_grid_pos() {
        let board = GameBoard::parse(Creature::elf(), Creature::goblin(), EXAMPLE_DATA);

        let mapping = [(7, (1, 2)), (17, (4, 3))];

        for (i, (x, y)) in &mapping {
            assert_eq!(board.grid_index(*x, *y), *i);
            assert_eq!(board.grid_coordinates(*i), (*x, *y));
        }
    }

    #[test]
    fn combat_1() {
        const COMBAT_DATA: &str = "\
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

        let mut board = GameBoard::parse(Creature::elf(), Creature::goblin(), COMBAT_DATA);

        assert_eq!(6, board.creature_count[&CreatureKind::Elf]);
        assert_eq!(2, board.creature_count[&CreatureKind::Goblin]);

        while !board.game_over() {
            board.turn();
        }

        assert_eq!(5, board.creature_count[&CreatureKind::Elf]);
        assert_eq!(0, board.creature_count[&CreatureKind::Goblin]);

        assert_eq!(37, board.rounds);

        assert_eq!(
            36334,
            board.rounds * board.remaining_hit_points().values().sum::<u32>()
        );
    }

    #[test]
    fn combat_2() {
        const DATA: &str = "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
";

        let mut board = GameBoard::parse(Creature::elf(), Creature::goblin(), DATA);

        assert_eq!(6, board.creature_count[&CreatureKind::Elf]);
        assert_eq!(3, board.creature_count[&CreatureKind::Goblin]);

        while !board.game_over() {
            board.turn();
        }

        assert_eq!(5, board.creature_count[&CreatureKind::Elf]);
        assert_eq!(0, board.creature_count[&CreatureKind::Goblin]);

        assert_eq!(46, board.rounds);

        assert_eq!(
            39514,
            board.rounds * board.remaining_hit_points().values().sum::<u32>()
        );
    }

    #[test]
    fn combat_3() {
        const DATA: &str = "\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

        let mut board = GameBoard::parse(Creature::elf(), Creature::goblin(), DATA);

        while !board.game_over() {
            board.turn();
        }

        assert_eq!(54, board.rounds);

        assert_eq!(
            28944,
            board.rounds * board.remaining_hit_points().values().sum::<u32>()
        );
    }

    #[test]
    fn combat_4() {
        const DATA: &str = "\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

        let mut board = GameBoard::parse(Creature::elf(), Creature::goblin(), DATA);

        while !board.game_over() {
            board.turn();
        }

        assert_eq!(20, board.rounds);
        assert_eq!(
            18740,
            board.rounds * board.remaining_hit_points().values().sum::<u32>()
        );
    }

    #[test]
    fn combat_full() {
        const DATA: &str = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

        let mut board = GameBoard::parse(Creature::elf(), Creature::goblin(), DATA);
        while !board.game_over() {
            board.turn();
        }

        assert_eq!(47, board.rounds);
        assert_eq!(
            27730,
            board.rounds * board.remaining_hit_points().values().sum::<u32>()
        );
    }

    #[test]
    fn movement() {
        const INITIAL: &str = "\
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########";

        // round 1
        // #########
        // #.G...G.#
        // #...G...#
        // #...E..G#
        // #.G.....#
        // #.......#
        // #G..G..G#
        // #.......#
        // #########

        // round 2
        // #########
        // #..G.G..#
        // #...G...#
        // #.G.E.G.#
        // #.......#
        // #G..G..G#
        // #.......#
        // #.......#
        // #########

        // round 3
        // #########
        // #.......#
        // #..GGG..#
        // #..GEG..#
        // #G..G...#
        // #......G#
        // #.......#
        // #.......#
        // #########

        let mut board = GameBoard::parse(Creature::elf(), Creature::goblin(), INITIAL);

        let check_tiles = |board: &GameBoard, creature_kind: CreatureKind, points: &[Point]| {
            for (x, y) in points {
                let tile = board.tile(*x, *y);

                let matching_kind = match tile {
                    Some(Tile::Creature(Creature { kind, .. })) if *kind == creature_kind => true,
                    _ => false,
                };

                assert!(
                    matching_kind,
                    "Expected {:?}, found: {:?}",
                    creature_kind, tile
                );
            }
        };

        board.turn();

        check_tiles(
            &board,
            CreatureKind::Goblin,
            &[
                (3, 2),
                (7, 2),
                (5, 3),
                (8, 4),
                (3, 5),
                (2, 7),
                (5, 7),
                (8, 4),
            ],
        );

        check_tiles(&board, CreatureKind::Elf, &[(5, 4)]);

        board.turn();

        check_tiles(
            &board,
            CreatureKind::Goblin,
            &[
                (4, 2),
                (6, 2),
                (5, 3),
                (3, 4),
                (7, 4),
                (2, 6),
                (5, 6),
                (8, 6),
            ],
        );

        check_tiles(&board, CreatureKind::Elf, &[(5, 4)]);

        board.turn();

        check_tiles(
            &board,
            CreatureKind::Goblin,
            &[
                (4, 3),
                (5, 3),
                (6, 3),
                (4, 4),
                (6, 4),
                (2, 5),
                (5, 5),
                (8, 6),
            ],
        );

        check_tiles(&board, CreatureKind::Elf, &[(5, 4)]);
    }

    #[test]
    fn part_2_1() {
        const DATA: &str = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

        let elf_count = |board: &GameBoard| board.creature_count[&CreatureKind::Elf];
        let mut board = GameBoard::parse(Creature::elf(), Creature::goblin(), DATA);

        let initial_elf_count = elf_count(&board);
        assert_eq!(2, initial_elf_count);

        while !board.game_over() {
            board.turn();
        }

        assert_eq!(0, elf_count(&board));

        let mut board = GameBoard::parse(
            Creature::elf_with_attack_power(14),
            Creature::goblin(),
            DATA,
        );

        while !board.game_over() {
            board.turn();
        }

        assert!(initial_elf_count > elf_count(&board));

        let mut board = GameBoard::parse(
            Creature::elf_with_attack_power(15),
            Creature::goblin(),
            DATA,
        );
        while !board.game_over() {
            board.turn();
        }

        assert_eq!(initial_elf_count, elf_count(&board));

        assert_eq!(29, board.rounds);
        assert_eq!(
            4988,
            board.rounds * board.remaining_hit_points().values().sum::<u32>()
        );
    }

    #[test]
    fn part_2_2() {
        const DATA: &str = "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

        let elf_count = |board: &GameBoard| board.creature_count[&CreatureKind::Elf];

        let mut board = GameBoard::parse(Creature::elf(), Creature::goblin(), DATA);
        let initial_elf_count = elf_count(&board);

        assert_eq!(6, initial_elf_count);

        while !board.game_over() {
            board.turn();
        }

        assert!(initial_elf_count > elf_count(&board));

        let mut board =
            GameBoard::parse(Creature::elf_with_attack_power(4), Creature::goblin(), DATA);

        while !board.game_over() {
            board.turn();
        }

        assert_eq!(initial_elf_count, elf_count(&board));
    }
}

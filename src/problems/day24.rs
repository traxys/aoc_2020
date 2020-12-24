use crate::DayContext;
use std::collections::HashSet;

type Input = Vec<Vec<Direction>>;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    a: bool,
    r: i64,
    c: i64,
}

impl Point {
    fn neighbour(&self, direction: Direction) -> Point {
        match direction {
            Direction::East => Point {
                c: self.c + 1,
                ..*self
            },
            Direction::West => Point {
                c: self.c - 1,
                ..*self
            },
            Direction::NorthEast => Point {
                a: !self.a,
                r: self.r - !self.a as i64,
                c: self.c + self.a as i64,
            },
            Direction::NorthWest => Point {
                a: !self.a,
                r: self.r - !self.a as i64,
                c: self.c - !self.a as i64,
            },
            Direction::SouthEast => Point {
                a: !self.a,
                r: self.r + self.a as i64,
                c: self.c + self.a as i64,
            },
            Direction::SouthWest => Point {
                a: !self.a,
                r: self.r + self.a as i64,
                c: self.c - !self.a as i64,
            },
        }
    }

    fn neighbours(&self) -> impl Iterator<Item = Point> {
        let directions = &[
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
        ];

        let s = *self;
        directions.iter().map(move |&dir| s.neighbour(dir))
    }
}

fn tile_from(slice: &[Direction], mut start: Point) -> Point {
    for &dir in slice {
        start = start.neighbour(dir);
    }

    start
}

fn colored(directions: &[Vec<Direction>]) -> HashSet<Point> {
    let mut colored = HashSet::new();
    let reference = Point {
        a: false,
        r: 0,
        c: 0,
    };
    for line in directions {
        let output = tile_from(&line, reference);
        if colored.contains(&output) {
            colored.remove(&output)
        } else {
            colored.insert(output)
        };
    }
    colored
}

pub fn part_1(input: Input) -> color_eyre::Result<String> {
    let colored = colored(&input);
    Ok(format!("Colored points: {}", colored.len()))
}

struct GameOfLife {
    active: HashSet<Point>,
}

impl GameOfLife {
    fn load(directions: &[Vec<Direction>]) -> Self {
        Self {
            active: colored(directions),
        }
    }
    fn neighbours(&self, point: Point) -> usize {
        point
            .neighbours()
            .filter(|neighbour| self.active.contains(neighbour))
            .count()
    }

    fn step(&mut self) {
        let mut new_active = HashSet::new();

        for &point in &self.active {
            // This point is black
            let neighbour_count = self.neighbours(point);
            if !(neighbour_count == 0 || neighbour_count > 2) {
                new_active.insert(point);
            }
            for neighbour in point.neighbours() {
                // Only do the white as we will have done the black in the outer loop
                if !self.active.contains(&neighbour) {
                    let neigbour_count = self.neighbours(neighbour);
                    if neigbour_count == 2 {
                        new_active.insert(neighbour);
                    }
                }
            }
        }

        self.active = new_active;
    }
}

pub fn part_2(directions: Input) -> color_eyre::Result<String> {
    let mut game_of_life = GameOfLife::load(&directions);
    for _ in 0..100 {
        game_of_life.step();
    }
    Ok(format!(
        "After a bit of game of life: {}",
        game_of_life.active.len()
    ))
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    context.parse_byte_lines(|line| {
        let mut line = line.as_ref();
        let mut directions = Vec::new();
        while !line.is_empty() {
            match line {
                [b'e', rest @ ..] => {
                    line = rest;
                    directions.push(Direction::East)
                }
                [b's', b'e', rest @ ..] => {
                    line = rest;
                    directions.push(Direction::SouthEast)
                }
                [b's', b'w', rest @ ..] => {
                    line = rest;
                    directions.push(Direction::SouthWest)
                }
                [b'n', b'w', rest @ ..] => {
                    line = rest;
                    directions.push(Direction::NorthWest)
                }
                [b'n', b'e', rest @ ..] => {
                    line = rest;
                    directions.push(Direction::NorthEast)
                }
                [b'w', rest @ ..] => {
                    line = rest;
                    directions.push(Direction::West)
                }
                _ => return Err(color_eyre::eyre::eyre!("Invalid line: {:?}", line)),
            }
        }
        Ok(directions)
    })
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}

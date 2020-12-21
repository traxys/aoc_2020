use crate::DayContext;
use std::collections::{HashMap, HashSet};

type Input = Vec<Tile>;

pub fn part_1(tiles: Input) -> color_eyre::Result<String> {
    let corners: u64 = get_corners(&tiles)
        .corners
        .keys()
        .map(|&TileId(id)| id as u64)
        .product();
    Ok(format!("Values at the corners: {}", corners))
}

pub fn part_2(tiles: Input) -> color_eyre::Result<String> {
    let _remade = remake(&tiles);
    todo!()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn same_axis(&self, other: Direction) -> bool {
        match self {
            Direction::North | Direction::South => {
                other == Direction::North || other == Direction::South
            }
            Direction::East | Direction::West => {
                other == Direction::East || other == Direction::West
            }
        }
    }

    fn flip(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Action {
    Flip { axis: Direction },
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone, Copy)]
struct Borders {
    north: u16,
    south: u16,
    east: u16,
    west: u16,
}

impl Borders {
    fn as_slice(&self) -> [u16; 4] {
        [self.north, self.south, self.east, self.west]
    }

    // Which border is it, and flipped?
    fn get_direction(&self, border: u16, tile_size: u8) -> Option<(Direction, bool)> {
        let flipped = flip(border, tile_size);

        if self.north == border {
            Some((Direction::North, false))
        } else if self.north == flipped {
            Some((Direction::North, true))
        } else if self.south == border {
            Some((Direction::South, false))
        } else if self.south == flipped {
            Some((Direction::South, true))
        } else if self.east == border {
            Some((Direction::East, false))
        } else if self.east == flipped {
            Some((Direction::East, true))
        } else if self.west == border {
            Some((Direction::West, false))
        } else if self.west == flipped {
            Some((Direction::West, true))
        } else {
            None
        }
    }

    fn rotate(&self, clockwise: bool) -> Borders {
        if clockwise {
            Borders {
                north: self.west,
                east: self.north,
                south: self.east,
                west: self.south,
            }
        } else {
            Borders {
                north: self.east,
                west: self.north,
                south: self.west,
                east: self.south,
            }
        }
    }

    fn flip(&self, axis: Direction, tile_size: u8) -> Borders {
        if axis == Direction::North || axis == Direction::South {
            Borders {
                north: flip(self.north, tile_size),
                south: flip(self.south, tile_size),
                east: self.west,
                west: self.east,
            }
        } else {
            Borders {
                east: flip(self.east, tile_size),
                west: flip(self.west, tile_size),
                north: self.south,
                south: self.north,
            }
        }
    }

    fn flip_fit(
        &self,
        a: u16,
        b: u16,
        tile_size: u8,
    ) -> (Direction, Direction, Borders, Vec<Action>) {
        let mut corner_actions = Vec::new();
        let mut fitted = self.clone();

        let (mut direction_a, flipped_a) = self.get_direction(a, tile_size).unwrap();
        if flipped_a {
            fitted = fitted.flip(direction_a, tile_size);
            corner_actions.push(Action::Flip { axis: direction_a });
        }

        let (direction_b, flipped_b) = fitted.get_direction(b, tile_size).unwrap();
        assert!(!direction_a.same_axis(direction_b));
        if flipped_b {
            fitted = fitted.flip(direction_b, tile_size);
            corner_actions.push(Action::Flip { axis: direction_b });
            direction_a = direction_a.flip();
        }

        (direction_a, direction_b, fitted, corner_actions)
    }

    fn fit_north_west(&self, a: u16, b: u16, tile_size: u8) -> (Borders, Vec<Action>) {
        let (direction_a, direction_b, mut fitted, mut corner_actions) =
            self.flip_fit(a, b, tile_size);

        // Now we have the piece correctly flipped, we just need to rotate it in place
        match direction_a {
            Direction::East => match direction_b {
                Direction::North => {
                    corner_actions.push(Action::CounterClockwise);
                    fitted = fitted.rotate(false);
                }
                Direction::South => {
                    corner_actions.push(Action::CounterClockwise);
                    fitted = fitted.rotate(false);
                    corner_actions.push(Action::CounterClockwise);
                    fitted = fitted.rotate(false);
                }
                _ => unreachable!(),
            },
            Direction::West => match direction_b {
                Direction::North => (),
                Direction::South => {
                    corner_actions.push(Action::Clockwise);
                    fitted = fitted.rotate(true);
                }
                _ => unreachable!(),
            },
            Direction::South => match direction_b {
                Direction::East => {
                    corner_actions.push(Action::CounterClockwise);
                    fitted = fitted.rotate(false);
                    corner_actions.push(Action::CounterClockwise);
                    fitted = fitted.rotate(false);
                }
                Direction::West => {
                    corner_actions.push(Action::Clockwise);
                    fitted = fitted.rotate(true);
                }
                _ => unreachable!(),
            },
            Direction::North => match direction_b {
                Direction::East => {
                    corner_actions.push(Action::CounterClockwise);
                    fitted = fitted.rotate(false);
                }
                Direction::West => (),
                _ => unreachable!(),
            },
        };

        (fitted, corner_actions)
    }

    fn fit_north_east(&self, a: u16, b: u16, tile_size: u8) -> (Borders, Vec<Action>) {
        let (direction_a, direction_b, mut fitted, mut corner_actions) =
            self.flip_fit(a, b, tile_size);

        // Now we have the piece correctly flipped, we just need to rotate it in place
        match direction_a {
            Direction::East => match direction_b {
                Direction::North => (),
                Direction::South => {
                    corner_actions.push(Action::CounterClockwise);
                    fitted = fitted.rotate(false);
                }
                _ => unreachable!(),
            },
            Direction::West => match direction_b {
                Direction::North => {
                    corner_actions.push(Action::Clockwise);
                    fitted = fitted.rotate(true);
                },
                Direction::South => {
                    corner_actions.push(Action::CounterClockwise);
                    fitted = fitted.rotate(false);
                    corner_actions.push(Action::CounterClockwise);
                    fitted = fitted.rotate(false);
                }
                _ => unreachable!(),
            },
            Direction::South => match direction_b {
                Direction::East => {
                    corner_actions.push(Action::CounterClockwise);
                    fitted = fitted.rotate(false);
                }
                Direction::West => {
                    corner_actions.push(Action::CounterClockwise);
                    fitted = fitted.rotate(false);
                    corner_actions.push(Action::CounterClockwise);
                    fitted = fitted.rotate(false);
                }
                _ => unreachable!(),
            },
            Direction::North => match direction_b {
                Direction::East => {
                    ()
                }
                Direction::West => {
                    corner_actions.push(Action::Clockwise);
                    fitted = fitted.rotate(true);
                },
                _ => unreachable!(),
            },
        };

        (fitted, corner_actions)
    }

    fn try_fit(&self, north: u16, west: u16, tile_size: u8) -> Option<(Borders, Vec<Action>)> {
        let (direction_north, direction_west, mut fitted, mut corner_actions) =
            self.flip_fit(north, west, tile_size);

        match direction_north {
            Direction::North => {
                if direction_west == Direction::West {
                    return None;
                }
            }
            Direction::West => {
                if direction_west != Direction::South {
                    return None;
                }
                corner_actions.push(Action::Clockwise);
                fitted = fitted.rotate(true);
            }
            Direction::East => {
                if direction_west == Direction::North {
                    return None;
                }
                corner_actions.push(Action::CounterClockwise);
                fitted = fitted.rotate(false);
            }
            Direction::South => {
                if direction_west == Direction::East {
                    return None;
                }
                corner_actions.push(Action::CounterClockwise);
                fitted = fitted.rotate(false);
                corner_actions.push(Action::CounterClockwise);
                fitted = fitted.rotate(false);
            }
        }

        Some((fitted, corner_actions))
    }
}

#[derive(Clone)]
pub struct Tile {
    id: TileId,
    tile: Vec<Vec<bool>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct TileId(u16);

fn encode_boolslice(s: &[bool]) -> u16 {
    s.iter().fold(0, |c, &b| c << 1 | b as u16)
}

fn flip(border: u16, size: u8) -> u16 {
    border.reverse_bits() >> (16 - size)
}

struct CornerLookup {
    borders: HashMap<TileId, Borders>,
    possible_joins: HashMap<u16, HashSet<TileId>>,
    corners: HashMap<TileId, HashSet<u16>>,
    border_tiles: HashMap<TileId, HashSet<u16>>,
}

fn remake(tiles: &[Tile]) -> Vec<Vec<(TileId, Vec<Action>)>> {
    let tile_size = tiles[0].tile.len() as u8;
    let CornerLookup {
        corners,
        borders,
        possible_joins,
        border_tiles,
    } = get_corners(tiles);

    let mut joined_tiles = Vec::new();

    let tiles_per_row = (tiles.len() as f64).sqrt() as usize;
    let mut first_row_picker = |index, tile_id| {
        if index < tiles_per_row - 1 {
            let tile_border = border_tiles.get(&tile_id).unwrap();
            assert_eq!(tile_border.len(), 1);
            *tile_border.into_iter().nth(0).unwrap()
        } else {
            let corner_borders = *borders.get(&tile_id).unwrap();
            let mut outside_borders = dbg!(border_tiles.get(&tile_id).unwrap()).iter();

            let (&border_a, &border_b) = (
                outside_borders.next().unwrap(),
                outside_borders.next().unwrap(),
            );

            dbg!(corner_borders);
            let (corner_borders, _) = corner_borders.fit_north_east(border_a, border_b, tile_size);
            corner_borders.north
        }
    };

    for (corner, outside_borders) in corners {
        let mut outside_borders = outside_borders.into_iter();
        let (border_a, border_b) = (
            outside_borders.next().unwrap(),
            outside_borders.next().unwrap(),
        );

        let corner_borders = *borders.get(&corner).unwrap();
        let (corner_borders, corner_actions) =
            corner_borders.fit_north_west(border_a, border_b, tile_size);

        let mut row = vec![(corner, corner_borders, corner_actions)];
        if place_tile(&mut first_row_picker, &mut row, &possible_joins, &borders, tile_size) {
            joined_tiles.push(row);
            break;
        }
    }
    if joined_tiles.len() == 0 {
        panic!("Did not find a correct corner")
    }

    fill_row(
        first_row_picker,
        &mut joined_tiles[0],
        tiles_per_row,
        &possible_joins,
        &borders,
        tile_size,
    );

    dbg!(&joined_tiles);

    todo!()
}

fn place_tile<F: FnMut(usize, TileId) -> u16>(
    north_picker: &mut F,
    current_row: &mut Vec<(TileId, Borders, Vec<Action>)>,
    possible_joins: &HashMap<u16, HashSet<TileId>>,
    borders: &HashMap<TileId, Borders>,
    tile_size: u8,
) -> bool {
    let (prev_id, prev_borders) = current_row
        .last()
        .map(|(id, borders, _)| (id, borders))
        .unwrap();
    let possible = possible_joins.get(&prev_borders.east).unwrap();
    assert_eq!(possible.len(), 2);

    let &linked = possible.iter().filter(|&x| x != prev_id).nth(0).unwrap();
    dbg!(prev_id, linked);

    let north = north_picker(current_row.len(), linked);
    let (fitted, actions) =
        match borders
            .get(&linked)
            .unwrap()
            .try_fit(north, prev_borders.east, tile_size)
        {
            Some(x) => x,
            None => return false,
        };
    current_row.push((linked, fitted, actions));
    true
}

fn fill_row<F: FnMut(usize, TileId) -> u16>(
    mut north_picker: F,
    current_row: &mut Vec<(TileId, Borders, Vec<Action>)>,
    tiles_per_row: usize,
    possible_joins: &HashMap<u16, HashSet<TileId>>,
    borders: &HashMap<TileId, Borders>,
    tile_size: u8,
) {
    while current_row.len() < tiles_per_row {
        assert!(place_tile(
            &mut north_picker,
            current_row,
            possible_joins,
            borders,
            tile_size,
        ));
    }
}

fn get_corners(tiles: &[Tile]) -> CornerLookup {
    let borders: HashMap<TileId, Borders> =
        tiles.iter().map(|tile| (tile.id, tile.borders())).collect();
    let tile_size = tiles[0].tile.len() as u8;

    let mut possible_joins = HashMap::new();
    for (&tile, borders) in &borders {
        for &border in borders.as_slice().iter() {
            let normal = possible_joins.entry(border).or_insert_with(HashSet::new);
            normal.insert(tile);

            let flipped = possible_joins
                .entry(flip(border, tile_size))
                .or_insert_with(HashSet::new);
            flipped.insert(tile);
        }
    }

    // HashMap[tile] -> possible border (u16 -> HashSet<u16>)
    let possible_border = possible_joins
        .iter()
        .fold(HashMap::new(), |mut tiles, (&border, t)| {
            if t.len() == 1 {
                let tile = t.iter().nth(0).copied().unwrap();
                let borders = tiles.entry(tile).or_insert_with(HashSet::new);
                if !borders.contains(&flip(border, tile_size)) {
                    borders.insert(border);
                }
            }
            tiles
        });

    CornerLookup {
        corners: possible_border
            .iter()
            .filter(|(_, borders)| borders.len() > 1)
            .map(|(&tile, borders)| (tile, borders.clone()))
            .collect(),
        borders,
        possible_joins,
        border_tiles: possible_border,
    }
}

impl Tile {
    fn border(&self, direction: Direction) -> Vec<bool> {
        match direction {
            Direction::North => self.tile.first().unwrap().clone(),
            Direction::South => self.tile.last().unwrap().clone(),
            Direction::West => self
                .tile
                .iter()
                .map(|line| *line.first().unwrap())
                .collect(),
            Direction::East => self.tile.iter().map(|line| *line.last().unwrap()).collect(),
        }
    }
    fn borders(&self) -> Borders {
        Borders {
            north: encode_boolslice(&self.border(Direction::North)),
            south: encode_boolslice(&self.border(Direction::South)),
            east: encode_boolslice(&self.border(Direction::East)),
            west: encode_boolslice(&self.border(Direction::West)),
        }
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "id = {}", self.id.0)?;
        for line in &self.tile {
            for px in line {
                write!(f, "{}", if *px { "#" } else { "." })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse_tile_line(
    tiles: &mut Vec<Tile>,
    current_tile: &mut Vec<Vec<bool>>,
    current_id: &mut Option<u16>,
    line: &str,
) -> color_eyre::Result<()> {
    if !line.is_empty() {
        if let Some(id) = line.strip_prefix("Tile ") {
            let id = id.trim_end_matches(":");
            if let Some(x) = current_id {
                let mut new_current = Vec::new();
                std::mem::swap(&mut new_current, current_tile);
                tiles.push(Tile {
                    id: TileId(*x),
                    tile: new_current,
                });
            }
            *current_id = Some(id.parse()?);
        } else {
            current_tile.push(
                line.as_bytes()
                    .iter()
                    .map(|c| match c {
                        b'#' => true,
                        b'.' => false,
                        _ => panic!("Invalid char in image"),
                    })
                    .collect(),
            )
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{get_corners, parse_tile_line, Tile};

    #[test]
    fn encode() {
        assert_eq!(
            super::encode_boolslice(&[
                true, false, true, true, false, false, false, true, true, false
            ]),
            0b1011000110
        );
    }

    #[test]
    fn corners_test() {
        let tiles = load_example();
        let corners: u64 = get_corners(&tiles)
            .corners
            .keys()
            .map(|&super::TileId(id)| id as u64)
            .product();

        assert_eq!(corners, 20899048083289)
    }

    /*
    #[test]
    fn remake_test() {
        let tiles = load_example();
        let _ = super::remake(&tiles);
        todo!()
    }*/

    fn load_example() -> Vec<Tile> {
        let input = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
"#;

        let mut tiles = Vec::new();
        let mut current_tile = Vec::new();
        let mut current_id = None;

        input.lines().for_each(|line| {
            parse_tile_line(&mut tiles, &mut current_tile, &mut current_id, line).unwrap();
        });

        tiles.push(Tile {
            id: super::TileId(current_id.unwrap()),
            tile: current_tile,
        });

        tiles
    }
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    let mut tiles = Vec::new();
    let mut current_tile = Vec::new();
    let mut current_id = None;

    context.accumulate_str_lines(|_, line| {
        parse_tile_line(&mut tiles, &mut current_tile, &mut current_id, line)
    })?;

    tiles.push(Tile {
        id: TileId(current_id.unwrap()),
        tile: current_tile,
    });

    Ok(tiles)
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}

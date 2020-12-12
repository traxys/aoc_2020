use crate::DayContext;

type Input = Ship;

pub fn part_1(mut ship: Input) -> color_eyre::Result<String> {
    ship.read_controls();
    Ok(format!("Distance travelled: {}", ship.distance_traveled()))
}

pub fn part_2(mut ship: Input) -> color_eyre::Result<String> {
    ship.read_waypoint_controls();
    Ok(format!("Distance travelled following waypoint: {}", ship.distance_traveled()))
}

#[derive(Clone, Copy)]
enum Control {
    North(u64),
    South(u64),
    East(u64),
    West(u64),
    Left(u8),
    Right(u8),
    Forward(u64),
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    East = 1,
    West = 3,
    South = 2,
    North = 0,
}

impl Direction {
    fn turn(&self, right: bool, amount: u8) -> Direction {
        let facing = *self as i8;
        let amount = if right { amount as i8 } else { -(amount as i8) };
        let mut new_facing = facing + amount;
        while new_facing < 0 {
            new_facing += 4
        }
        new_facing = new_facing % 4;

        match new_facing {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!("invalid direction after %4"),
        }
    }
}

pub struct Ship {
    controls: Vec<Control>,
    facing: Direction,
    position: (i64, i64),
    waypoint: (i64, i64),
}

impl Ship {
    fn new(controls: Vec<Control>) -> Self {
        Self {
            controls,
            facing: Direction::East,
            position: (0, 0),
            waypoint: (10, 1),
        }
    }

    fn distance_traveled(&self) -> u64 {
        self.position.0.abs() as u64 + self.position.1.abs() as u64
    }

    fn read_controls(&mut self) {
        for i in 0..self.controls.len() {
            match self.controls[i] {
                Control::North(u) => self.move_direction(Direction::North, u),
                Control::West(u) => self.move_direction(Direction::West, u),
                Control::East(u) => self.move_direction(Direction::East, u),
                Control::South(u) => self.move_direction(Direction::South, u),
                Control::Forward(u) => self.move_direction(self.facing, u),
                Control::Left(u) => self.facing = self.facing.turn(false, u),
                Control::Right(u) => self.facing = self.facing.turn(true, u),
            }
        }
    }

    fn move_direction(&mut self, dir: Direction, units: u64) {
        let units = units as i64;
        match dir {
            Direction::East => self.position.0 += units,
            Direction::West => self.position.0 -= units,
            Direction::North => self.position.1 += units,
            Direction::South => self.position.1 -= units,
        }
    }

    fn move_waypoint(&mut self, dir: Direction, units: u64) {
        let units = units as i64;
        match dir {
            Direction::East => self.waypoint.0 += units,
            Direction::West => self.waypoint.0 -= units,
            Direction::North => self.waypoint.1 += units,
            Direction::South => self.waypoint.1 -= units,
        }
    }

    fn move_to_waypoint(&mut self, amount: u64) {
        self.position.0 += self.waypoint.0 * amount as i64;
        self.position.1 += self.waypoint.1 * amount as i64;
    }

    fn rotate_waypoint_right(&mut self, amount: u8) {
        for _ in 0..amount {
            self.waypoint = (self.waypoint.1, -self.waypoint.0)
        }
    }

    fn rotate_waypoint_left(&mut self, amount: u8) {
        for _ in 0..amount {
            self.waypoint = (-self.waypoint.1, self.waypoint.0)
        }
    }

    fn read_waypoint_controls(&mut self) {
        for i in 0..self.controls.len() {
            match self.controls[i] {
                Control::North(u) => self.move_waypoint(Direction::North, u),
                Control::West(u) => self.move_waypoint(Direction::West, u),
                Control::East(u) => self.move_waypoint(Direction::East, u),
                Control::South(u) => self.move_waypoint(Direction::South, u),
                Control::Forward(u) => self.move_to_waypoint(u),
                Control::Left(u) => self.rotate_waypoint_left(u),
                Control::Right(u) => self.rotate_waypoint_right(u),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Control, Ship};

    const CONTROLS: &[Control] = &[
        Control::Forward(10),
        Control::North(3),
        Control::Forward(7),
        Control::Right(1),
        Control::Forward(11),
    ];

    #[test]
    fn day12_pt1() {
        let mut ship = Ship::new(CONTROLS.to_owned());
        ship.read_controls();
        assert_eq!(ship.distance_traveled(), 25);
    }

    #[test]
    fn day12_pt2() {
        let mut ship = Ship::new(CONTROLS.to_owned());
        ship.read_waypoint_controls();
        assert_eq!(ship.distance_traveled(), 286);
    }
}

pub fn parsing(ctx: &mut DayContext) -> color_eyre::Result<Input> {
    let controls = ctx.parse_lines(|line| match &line[0..1] {
        "F" => Ok(Control::Forward(line[1..].parse()?)),
        "N" => Ok(Control::North(line[1..].parse()?)),
        "S" => Ok(Control::South(line[1..].parse()?)),
        "E" => Ok(Control::East(line[1..].parse()?)),
        "W" => Ok(Control::West(line[1..].parse()?)),
        "L" => Ok(Control::Left((line[1..].parse::<u16>()? / 90) as u8)),
        "R" => Ok(Control::Right((line[1..].parse::<u16>()? / 90) as u8)),
        invalid => color_eyre::eyre::bail!("Invalid direction: {}", invalid),
    })?;

    Ok(Ship::new(controls))
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}

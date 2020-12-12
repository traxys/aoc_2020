use std::cell::Cell;

use crate::DayContext;

type Input = AeroportGame;

pub fn part_1(mut seats: Input) -> color_eyre::Result<String> {
    while seats.step() {}
    Ok(format!("Free seats at the end: {}", seats.count_full()))
}

pub fn part_2(mut seats: Input) -> color_eyre::Result<String> {
    while seats.step_visible() {}
    Ok(format!("Free seats at the end: {}", seats.count_full()))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum State {
    Floor,
    Empty,
    Full,
}

pub struct AeroportGame {
    grid: Vec<Vec<Cell<State>>>,
}

impl std::fmt::Debug for AeroportGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for seat in line {
                let seat = match seat.get() {
                    State::Floor => ".",
                    State::Empty => "L",
                    State::Full => "#",
                };
                write!(f, "{}", seat)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl AeroportGame {
    pub fn step(&mut self) -> bool {
        let new_grid = self.grid.clone();
        let mut changed = false;

        for (i, line) in new_grid.iter().enumerate() {
            for (j, seat) in line.iter().enumerate() {
                let new_state = match seat.get() {
                    State::Floor => State::Floor,
                    State::Empty if self.neighbours(i, j) == 0 => {
                        changed = true;
                        State::Full
                    }
                    State::Full if self.neighbours(i, j) >= 4 => {
                        changed = true;
                        State::Empty
                    }
                    s => s,
                };
                seat.set(new_state);
            }
        }

        self.grid = new_grid;
        changed
    }

    pub fn step_visible(&mut self) -> bool {
        let new_grid = self.grid.clone();
        let mut changed = false;

        for (i, line) in new_grid.iter().enumerate() {
            for (j, seat) in line.iter().enumerate() {
                let new_state = match seat.get() {
                    State::Floor => State::Floor,
                    State::Empty if self.visible(i, j) == 0 => {
                        changed = true;
                        State::Full
                    }
                    State::Full if self.visible(i, j) >= 5 => {
                        changed = true;
                        State::Empty
                    }
                    s => s,
                };
                seat.set(new_state);
            }
        }

        self.grid = new_grid;
        changed
    }

    fn count_full(&self) -> usize {
        self.grid
            .iter()
            .map(|l| l.iter().filter(|s| s.get() == State::Full).count())
            .sum()
    }

    fn neighbours(&self, i: usize, j: usize) -> usize {
        let mut count = 0;

        let istart = if i == 0 { 1 } else { 0 };
        let iend = if i == self.grid.len() - 1 { 2 } else { 3 };

        let jstart = if j == 0 { 1 } else { 0 };
        let jend = if j == self.grid[0].len() - 1 { 2 } else { 3 };

        for di in istart..iend {
            for dj in jstart..jend {
                // Same cell
                if di == 1 && dj == 1 {
                    continue;
                }

                match self.grid[i + di - 1][j + dj - 1].get() {
                    State::Full => count += 1,
                    _ => (),
                }
            }
        }

        count
    }

    fn visible(&self, i: usize, j: usize) -> usize {
        let mut count = 0;

        for di in -1i64..=1 {
            for dj in -1..=1 {
                if di == 0 && dj == 0 {
                    continue;
                }
                for k in 1i64.. {
                    let vi = i as i64 + (di * k);
                    if vi < 0 || vi as usize >= self.grid.len() {
                        break;
                    }
                    let vi = vi as usize;

                    let vj = j as i64 + (dj * k);
                    if vj < 0 || vj as usize >= self.grid[0].len() {
                        break;
                    }
                    let vj = vj as usize;

                    match self.grid[vi][vj].get() {
                        State::Full => {
                            count += 1;
                            break;
                        }
                        State::Empty => break,
                        State::Floor => continue,
                    }
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::{AeroportGame, State};
    use std::cell::Cell;

    fn simple() -> AeroportGame {
        let mut grid = Vec::new();
        for _ in 0..10 {
            grid.push(vec![Cell::new(State::Empty); 10]);
        }
        grid[0][1].set(State::Floor);
        grid[0][4].set(State::Floor);
        grid[0][7].set(State::Floor);

        grid[1][7].set(State::Floor);

        grid[2][1].set(State::Floor);
        grid[2][3].set(State::Floor);
        grid[2][5].set(State::Floor);
        grid[2][6].set(State::Floor);
        grid[2][8].set(State::Floor);
        grid[2][9].set(State::Floor);

        grid[3][4].set(State::Floor);
        grid[3][7].set(State::Floor);

        grid[4][1].set(State::Floor);
        grid[4][4].set(State::Floor);
        grid[4][7].set(State::Floor);

        grid[5][1].set(State::Floor);
        grid[5][7].set(State::Floor);

        grid[6][0].set(State::Floor);
        grid[6][1].set(State::Floor);
        grid[6][3].set(State::Floor);
        grid[6][5].set(State::Floor);
        grid[6][6].set(State::Floor);
        grid[6][7].set(State::Floor);
        grid[6][8].set(State::Floor);
        grid[6][9].set(State::Floor);

        grid[8][1].set(State::Floor);
        grid[8][8].set(State::Floor);

        grid[9][1].set(State::Floor);
        grid[9][7].set(State::Floor);

        AeroportGame { grid }
    }

    #[test]
    fn day11_simple() {
        let mut simple = simple();

        println!("{:?}", simple);
        while simple.step() {
            println!("{:?}", simple);
        }

        assert_eq!(simple.count_full(), 37)
    }

    #[test]
    fn day11_simple_visible() {
        let mut simple = simple();

        println!("{:?}", simple);
        let mut i = 0;
        while simple.step_visible() {
            i += 1;
            if i > 6 {
                panic!("too many iteration");
            }
            println!("{:?}", simple);
        }

        assert_eq!(simple.count_full(), 26)
    }
}

pub fn parsing(ctx: &mut DayContext) -> color_eyre::Result<Input> {
    let grid = ctx.parse_byte_lines(|line| {
        line.iter()
            .map(|i| match i {
                b'.' => Ok(Cell::new(State::Floor)),
                b'L' => Ok(Cell::new(State::Empty)),
                _ => color_eyre::eyre::bail!("No such seat kind"),
            })
            .collect()
    })?;

    Ok(AeroportGame { grid })
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}

use crate::DayContext;

type Input<'i> = &'i [(u8, u8)];

fn seat_id((row, col): (u8, u8)) -> u64 {
    (row as u64 * 8) + col as u64
}

pub fn part_1(seat: Input) -> color_eyre::Result<String> {
    let max_id = seat
        .iter()
        .map(|&x| seat_id(x))
        .max()
        .ok_or_else(|| color_eyre::eyre::eyre!("No passes provided"))?;

    Ok(format!("Highest seat id is: {}", max_id))
}

pub fn part_2(seat: Input) -> color_eyre::Result<String> {
    let mut ids: Vec<u64> = seat.iter().map(|&x| seat_id(x)).collect();
    ids.sort_unstable();

    let missing_id = ids
        .windows(2)
        .filter(|x| x[0] + 2 == x[1])
        .map(|x| x[0] + 1)
        .nth(0)
        .ok_or_else(|| color_eyre::eyre::eyre!("Did not find a missing id"))?;

    Ok(format!("Missing id is: {}", missing_id))
}

#[derive(Clone, Copy, Debug)]
enum RowSelection {
    Front,
    Back,
}

#[derive(Clone, Copy, Debug)]
enum ColSelection {
    Right,
    Left,
}

struct BoardingPass {
    row: [RowSelection; 7],
    col: [ColSelection; 3],
}

impl BoardingPass {
    fn seat(&self) -> (u8, u8) {
        let mut row = 0..128;
        for r in &self.row {
            let half = (row.start + row.end) / 2;
            match r {
                RowSelection::Back => row.start = half,
                RowSelection::Front => row.end = half,
            }
        }

        let mut col = 0..8;
        for c in &self.col {
            let half = (col.start + col.end) / 2;
            match c {
                ColSelection::Right => col.start = half,
                ColSelection::Left => col.end = half,
            }
        }

        (row.start, col.start)
    }
}

#[cfg(test)]
mod test {
    use super::{BoardingPass, ColSelection, RowSelection};

    #[test]
    fn test_seat() {
        let pass = BoardingPass {
            row: [
                RowSelection::Front,
                RowSelection::Back,
                RowSelection::Front,
                RowSelection::Back,
                RowSelection::Back,
                RowSelection::Front,
                RowSelection::Front,
            ],
            col: [ColSelection::Right, ColSelection::Left, ColSelection::Right],
        };

        assert_eq!(pass.seat(), (44, 5))
    }
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Vec<(u8, u8)>> {
    context.parse_byte_lines(|line| {
        if line.len() != 10 {
            color_eyre::eyre::bail!("Line of invalid length: {}", line)
        }
        let mut row = [RowSelection::Front; 7];
        for (row, _) in row.iter_mut().zip(line.iter()).filter(|&(_, &l)| l == b'B') {
            *row = RowSelection::Back;
        }

        let mut col = [ColSelection::Left; 3];
        for (col, _) in col
            .iter_mut()
            .zip(line[7..].iter())
            .filter(|&(_, &l)| l == b'R')
        {
            *col = ColSelection::Right;
        }

        Ok(BoardingPass { row, col }.seat())
    })
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input.as_ref(), part_1, part_2)
}

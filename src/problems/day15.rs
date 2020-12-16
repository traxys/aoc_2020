use crate::DayContext;
use std::collections::HashMap;

type Input = Vec<u64>;

pub fn part_1(start: Input) -> color_eyre::Result<String> {
    let mut seq = Sequence::new(&start);
    Ok(format!("The 2020 term is: {}", seq.step_until(2020)))
}

struct Sequence {
    seen: HashMap<u64, u64>,
    idx: u64,
    last: u64,
}

impl Sequence {
    fn new(initial: &[u64]) -> Self {
        let mut seen = HashMap::new();
        for (i, initial) in initial.iter().enumerate() {
            seen.insert(*initial, i as u64);
        }

        let last = *initial.last().unwrap();
        seen.remove(&last);

        Self {
            seen,
            last,
            idx: initial.len() as u64 - 1,
        }
    }

    fn step(&mut self) -> u64 {
        match self.seen.get(&self.last).copied() {
            None => {
                self.seen.insert(self.last, self.idx);
                self.last = 0;
                self.idx += 1;
            }
            Some(last_idx) => {
                self.seen.insert(self.last, self.idx);
                let diff = self.idx - last_idx;
                self.last = diff;
                self.idx += 1;
            }
        }
        self.last
    }

    fn step_until(&mut self, i: u64) -> u64 {
        while self.idx != i - 1 {
            self.step();
        }
        self.last
    }
}

#[cfg(test)]
mod test {
    use super::Sequence;

    #[test]
    fn day15_simplest_p1() {
        let mut seq = Sequence::new(&[0, 3, 6]);
        assert_eq!(seq.step(), 0);
        assert_eq!(seq.step(), 3);
        assert_eq!(seq.step(), 3);
        assert_eq!(seq.step(), 1);
        assert_eq!(seq.step(), 0);
        assert_eq!(seq.step(), 4);
        assert_eq!(seq.step(), 0);
    }

    #[test]
    fn day15_2020_p1() {
        let mut seq = Sequence::new(&[1, 3, 2]);
        assert_eq!(seq.step_until(2020), 1);
    }

    #[test]
    fn day15_2020_2_p1() {
        let mut seq = Sequence::new(&[3, 1, 2]);
        assert_eq!(seq.step_until(2020), 1836);
    }
}

pub fn part_2(start: Input) -> color_eyre::Result<String> {
    let mut seq = Sequence::new(&start);
    Ok(format!("The 2020 term is: {}", seq.step_until(30000000)))
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    context
        .read_line()?
        .split(",")
        .map(|s| Ok(s.parse()?))
        .collect()
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}

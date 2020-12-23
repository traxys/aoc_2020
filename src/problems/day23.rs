use crate::DayContext;
use std::convert::TryInto;

type Input = [u32; 9];

pub fn part_1(cups: Input) -> color_eyre::Result<String> {
    let mut cups = Cups::new(&cups);
    for _ in 0..100 {
        cups.round();
    }
    let ring = cups.ring_starting_at(1);
    let output: String = ring.iter().skip(1).map(|x| x.to_string()).collect();
    Ok(format!("Ring after 100 moves is: {}", output))
}

pub fn part_2(first_cups: Input) -> color_eyre::Result<String> {
    let mut cups: Vec<u32> = (1..).take(1000000).collect();
    for (c, &f) in cups.iter_mut().zip(first_cups.iter()) {
        *c = f as u32;
    }

    let mut cups = Cups::new(&cups);
    for _i in 0..10000000 {
        cups.round();
    }

    let after_1 = cups.ring[1];
    let after_after1 = cups.ring[after_1];

    Ok(format!(
        "Star cups are: {}",
        after_after1 as u64 * after_1 as u64
    ))
}

#[derive(Debug)]
pub struct Cups {
    ring: Vec<usize>,
    current: usize,
    max: usize,
}

impl Cups {
    fn new(ring_list: &[u32]) -> Self {
        let mut ring = vec![0; ring_list.len() + 1];
        for (&i, &j) in ring_list.iter().zip(ring_list.iter().skip(1)) {
            ring[i as usize] = j as usize;
        }
        ring[*ring_list.last().unwrap() as usize] = *ring_list.first().unwrap() as usize;
        Self {
            current: ring_list[0] as usize,
            max: ring_list.len(),
            ring,
        }
    }

    fn take_cups(&mut self, idx: usize) -> [usize; 3] {
        let mut cups = [0; 3];

        for cup in &mut cups {
            let target = self.ring[idx];
            let next = self.ring[target];
            *cup = target;
            self.ring[self.current] = next;
        }

        cups
    }

    /// .... -> after -> next -> ....
    /// .... -> after -> cup[0] -> cup[1] -> cup[2] -> next -> ...
    fn insert_cups(&mut self, cups: [usize; 3], after: usize) {
        let next = self.ring[after];
        self.ring[after] = cups[0];
        self.ring[cups[0]] = cups[1];
        self.ring[cups[1]] = cups[2];
        self.ring[cups[2]] = next;
    }

    fn round(&mut self) {
        let cups = self.take_cups(self.current);
        let dest = self.destination(cups);
        self.insert_cups(cups, dest);
        self.current = self.ring[self.current];
    }

    fn destination(&self, invalid: [usize; 3]) -> usize {
        let mut current = self.current;

        loop {
            let target = if current == 1 { self.max } else { current - 1 };

            if !invalid.contains(&target) {
                break target;
            } else {
                current = target;
            }
        }
    }

    fn ring_starting_at(&self, start: usize) -> Vec<usize> {
        let mut r = vec![start];
        loop {
            let &point = r.last().unwrap();
            let next = self.ring[point];
            if next == start {
                break r;
            } else {
                r.push(next)
            }
        }
    }

    #[cfg(test)]
    fn ring_starting_at_current(&self) -> Vec<usize> {
        self.ring_starting_at(self.current)
    }
}

#[cfg(test)]
mod test {
    use super::Cups;

    fn load_example() -> Cups {
        Cups::new(&[3, 8, 9, 1, 2, 5, 4, 6, 7])
    }

    #[test]
    fn example_first_following() {
        let mut input = load_example();
        let following = input.take_cups(input.current);
        assert_eq!(following, [8, 9, 1]);
    }

    #[test]
    fn example_first_destination() {
        let mut input = load_example();
        let following = input.take_cups(input.current);
        let dest = input.destination(following);
        assert_eq!(dest, 2);
    }

    #[test]
    fn one_round() {
        let mut input = load_example();
        input.round();
        assert_eq!(
            input.ring_starting_at_current(),
            [2, 8, 9, 1, 5, 4, 6, 7, 3]
        )
    }

    #[test]
    fn example_rounds() {
        let mut input = load_example();
        for _ in 0..10 {
            input.round();
        }
        assert_eq!(
            input.ring_starting_at_current(),
            [8, 3, 7, 4, 1, 9, 2, 6, 5]
        )
    }

    #[test]
    fn example_100() {
        let mut input = load_example();
        for _ in 0..100 {
            input.round();
        }
        assert_eq!(input.ring_starting_at(1), [1, 6, 7, 3, 8, 4, 5, 2, 9])
    }
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    let ring = context.read_line()?;
    let ring: Vec<_> = ring.as_bytes().iter().map(|c| (c - b'0') as u32).collect();
    let array: [u32; 9] = ring
        .try_into()
        .map_err(|v: Vec<_>| color_eyre::eyre::eyre!("Ring is of invalid size: {}", v.len()))?;

    Ok(array)
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}

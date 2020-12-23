use crate::DayContext;
use std::{
    convert::{TryFrom, TryInto},
    ops::Sub,
};
use tinyvec::{Array, ArrayVec};

pub trait VecLike<T>: AsRef<[T]> + AsMut<[T]> {
    fn insert(&mut self, index: usize, element: T);
    fn remove(&mut self, index: usize) -> T;
}

impl<A: Array> VecLike<A::Item> for ArrayVec<A> {
    fn insert(&mut self, index: usize, element: A::Item) {
        self.insert(index, element)
    }
    fn remove(&mut self, index: usize) -> A::Item {
        self.remove(index)
    }
}

type Input = [u8; 9];

pub fn part_1(cups: Input) -> color_eyre::Result<String> {
    let mut cups = Cups::new(ArrayVec::from(cups));
    for _ in 0..100 {
        cups.round();
    }
    let ring = cups.ring_starting_at(1);
    let output: String = ring.iter().skip(1).map(|x| x.to_string()).collect();
    Ok(format!("Ring after 100 moves is: {}", output))
}

pub fn part_2(_: Input) -> color_eyre::Result<String> {
    todo!()
}

#[derive(Debug)]
pub struct Cups<T: Copy + PartialEq + Eq + From<u8> + Sub<T, Output = T>, V: VecLike<T>> {
    ring: V,
    current: T,
    max: T,
}

impl<
        T: Copy + PartialEq + Eq + Default + TryFrom<usize> + From<u8> + Sub<T, Output = T>,
        V: VecLike<T>,
    > Cups<T, V>
{
    fn new(ring: V) -> Self {
        Self {
            current: ring.as_ref()[0],
            max: match T::try_from(ring.as_ref().len()) {
                Ok(v) => v,
                Err(_) => panic!("Len does not fit in T"),
            },
            ring,
        }
    }
    fn following(&self, idx: usize) -> [T; 3] {
        let mut arr = [T::default(); 3];
        for i in 0..3 {
            arr[i] = self.ring.as_ref()[(idx + 1 + i) % 9];
        }
        arr
    }

    fn following_current(&self) -> [T; 3] {
        self.following(self.idx_of(self.current))
    }

    fn idx_of(&self, target: T) -> usize {
        self.ring
            .as_ref()
            .iter()
            .enumerate()
            .find(|(_, &x)| x == target)
            .unwrap()
            .0
    }

    fn round(&mut self) {
        let following_current = self.following_current();
        for &x in following_current.iter() {
            self.ring.remove(self.idx_of(x));
        }
        let dest = self.locate_destination(following_current);
        for &x in following_current.iter().rev() {
            self.ring.insert(dest + 1, x)
        }
        self.current = self.ring.as_ref()[(self.idx_of(self.current) + 1) % 9]
    }

    fn locate_destination(&self, invalid: [T; 3]) -> usize {
        let mut current = self.current;
        let target = loop {
            let target = if current == T::from(1) {
                self.max
            } else {
                current - T::from(1)
            };

            if !invalid.contains(&target) {
                break target;
            } else {
                current = target;
            }
        };
        self.idx_of(target)
    }

    fn ring_starting_at(&self, start: T) -> Vec<T> {
        let mut r = vec![T::default(); self.ring.as_ref().len()];
        let current = self.idx_of(start);
        let end = &self.ring.as_ref()[current..];
        r[..end.len()].copy_from_slice(end);
        r[end.len()..].copy_from_slice(&self.ring.as_ref()[..current]);
        r
    }

    #[cfg(test)]
    fn ring_starting_at_current(&self) -> Vec<T> {
        self.ring_starting_at(self.current)
    }
}

#[cfg(test)]
mod test {
    use super::Cups;
    use tinyvec::ArrayVec;

    fn load_example() -> Cups<u8, ArrayVec<[u8; 9]>> {
        Cups::new(tinyvec::array_vec![3, 8, 9, 1, 2, 5, 4, 6, 7])
    }

    #[test]
    fn example_first_following() {
        let input = load_example();
        let following = input.following_current();
        assert_eq!(following, [8, 9, 1]);
    }

    #[test]
    fn example_first_destination() {
        let input = load_example();
        let dest = input.locate_destination(input.following_current());
        assert_eq!(dest, 4);
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
    let ring: Vec<_> = ring.as_bytes().iter().map(|c| c - b'0').collect();
    let array: [u8; 9] = ring
        .try_into()
        .map_err(|v: Vec<_>| color_eyre::eyre::eyre!("Ring is of invalid size: {}", v.len()))?;

    Ok(array)
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}

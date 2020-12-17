use crate::DayContext;
use itertools::{izip, Itertools};
use std::{collections::HashSet, ops::Range};

type Input = ConwayCube;

pub fn part_1(mut cube: Input) -> color_eyre::Result<String> {
    cube.inc_dim();

    for _ in 0..6 {
        cube.step()
    }
    Ok(format!("Active cubes after boot: {}", cube.active()))
}

pub fn part_2(mut cube: Input) -> color_eyre::Result<String> {
    cube.inc_dim();
    cube.inc_dim();

    for _ in 0..6 {
        cube.step()
    }
    Ok(format!("Active hyper cubes after boot: {}", cube.active()))
}

// The spread is one more than the largest
// It would be nicer with const generics
pub struct ConwayCube {
    dim: usize,
    active: HashSet<Vec<i64>>,
    spreads: Vec<Range<i64>>,
}

impl ConwayCube {
    fn active(&self) -> usize {
        self.active.len()
    }

    fn inc_dim(&mut self) {
        self.dim += 1;
        self.spreads.push(-1..2);
        let mut new_active = HashSet::new();
        std::mem::swap(&mut new_active, &mut self.active);
        self.active = new_active
            .into_iter()
            .map(|mut p| {
                p.push(0);
                p
            })
            .collect();
    }

    fn neighbours(&self, point: &[i64]) -> usize {
        (0..self.dim)
            .map(|_| -1..2)
            .multi_cartesian_product()
            .filter(|d| !d.iter().all(|&c| c == 0))
            .map(|mut d| {
                for (dc, c) in d.iter_mut().zip(point.iter()) {
                    *dc += c;
                }
                d
            })
            .filter(|point| self.active.contains(point))
            .count()
    }

    fn step(&mut self) {
        let mut new_active = HashSet::new();

        let mut range_extend = vec![(false, false); self.dim];

        for point in self.spreads.clone().into_iter().multi_cartesian_product() {
            if self.active.contains(&point) {
                let neighbours = self.neighbours(&point);
                if neighbours == 2 || neighbours == 3 {
                    new_active.insert(point);
                }
            } else {
                if self.neighbours(&point) == 3 {
                    for (range, spread, &coord) in
                        izip!(range_extend.iter_mut(), self.spreads.iter(), point.iter())
                    {
                        range.0 |= coord == spread.start;
                        range.1 |= coord == spread.end - 1;
                    }

                    new_active.insert(point);
                }
            }
        }

        for (spread, extend) in self.spreads.iter_mut().zip(range_extend) {
            spread.start -= extend.0 as i64;
            spread.end += extend.1 as i64;
        }

        self.active = new_active;
    }
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    let mut active = HashSet::new();
    let mut x_spread = -1..0;
    let mut y_spread = -1..1;

    context.accumulate_byte_lines(|y, line| {
        line.iter()
            .enumerate()
            .filter_map(|(x, &c)| if c == b'#' { Some(x) } else { None })
            .for_each(|x| {
                active.insert(vec![x as i64, y as i64]);
            });
        x_spread.end = line.len() as i64 + 1;
        y_spread.end += 1;
        Ok(())
    })?;

    Ok(ConwayCube {
        active,
        dim: 2,
        spreads: vec![x_spread, y_spread],
    })
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}

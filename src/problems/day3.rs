use std::collections::HashSet;

use crate::DayContext;

pub fn part_1(slope: &Slope) -> color_eyre::Result<String> {
    let collisions = (0..slope.height)
        .map(|y| (3 * y % slope.width, y))
        .filter(|&(x, y)| slope.collides(x, y))
        .count();

    Ok(format!(
        "On the line (3, 1) there were {} collisions",
        collisions
    ))
}

pub fn part_2(slope: &Slope) -> color_eyre::Result<String> {
    let (slope1, slope2, slope3, slope4) = (0..slope.height)
        .map(|y| {
            (
                y % slope.width,
                3 * y % slope.width,
                5 * y % slope.width,
                7 * y % slope.width,
                y,
            )
        })
        .fold((0, 0, 0, 0), |(s1, s2, s3, s4), (x1, x2, x3, x4, y)| {
            (
                s1 + slope.collides(x1, y) as usize,
                s2 + slope.collides(x2, y) as usize,
                s3 + slope.collides(x3, y) as usize,
                s4 + slope.collides(x4, y) as usize,
            )
        });
    let slope5 = (0..(slope.height / 2))
        .map(|x| (x % slope.width, x * 2))
        .filter(|&(x, y)| slope.collides(x, y))
        .count();

    Ok(format!(
        "All collisions: {}",
        slope1 * slope2 * slope3 * slope4 * slope5
    ))
}

pub struct Slope {
    pub width: usize,
    pub height: usize,
    trees: HashSet<(usize, usize)>,
}

impl Slope {
    pub fn collides(&self, x: usize, y: usize) -> bool {
        self.trees.contains(&(x, y))
    }
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Slope> {
    let mut trees = HashSet::new();
    let mut height = 0;
    let mut width = 0;

    context.accumulate_byte_lines(|line_number, line| {
        for (index, &tree) in line.iter().enumerate() {
            if tree == b'#' {
                trees.insert((index, line_number));
            }
        }
        width = line.len();
        height += 1;

        Ok(())
    })?;

    Ok(Slope {
        width,
        height,
        trees,
    })
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(&input, part_1, part_2)
}

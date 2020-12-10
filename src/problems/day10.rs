use std::{
    collections::{HashMap, HashSet},
    iter::{repeat, repeat_with},
};

use nalgebra::DMatrix;
use petgraph::algo::all_simple_paths;

use crate::DayContext;

type Input<'i> = &'i mut [u64];

fn differences(input: &[u64]) -> (u64, u64) {
    let (one, two, three) = match input[0] {
        1 => (1, 0, 0),
        2 => (0, 1, 0),
        3 => (0, 0, 1),
        _ => panic!("Invalid jolt difference"),
    };
    let (one, _, three) = input.iter().zip(input.iter().skip(1)).fold(
        (one, two, three + 1),
        |(one, two, three), (&current, &next)| match next - current {
            1 => (one + 1, two, three),
            2 => (one, two + 1, three),
            3 => (one, two, three + 1),
            _ => panic!("Invalid jolt difference"),
        },
    );

    (one, three)
}

pub fn part_1(input: Input) -> color_eyre::Result<String> {
    input.sort();
    let (one, three) = differences(input);
    Ok(format!("One * three jolt: {}", one * three))
}

fn chain_count(adaptaters: &[u64]) -> u64 {
    let mut matrix: DMatrix<u64> =
        nalgebra::base::DMatrix::repeat(adaptaters.len() + 2, adaptaters.len() + 2, 0);
    let mut seen = HashMap::new();
    seen.insert(0, 0);

    for (i, &adapter) in adaptaters.iter().enumerate() {
        seen.insert(adapter, i + 1);
        for j in 1..=3 {
            if adapter >= j {
                if let Some(&idx) = seen.get(&(adapter - j)) {
                    matrix[(idx, i + 1)] = 1;
                }
            }
        }
    }

    let device_jolt = *adaptaters.last().unwrap() + 3;
    seen.insert(device_jolt, adaptaters.len() + 1);
    for j in 1..=3 {
        if let Some(&idx) = seen.get(&(device_jolt - j)) {
            matrix[(idx, adaptaters.len() + 1)] = 1;
        }
    }

    let mut adj = matrix.clone();

    let mut count = 0;
    for _ in 0..adaptaters.len() {
        count += adj[(0, adaptaters.len())];
        adj *= &matrix;
    }

    count
}

#[cfg(test)]
mod test {
    use super::{chain_count, differences};

    const SIMPLE: &[u64] = &[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    #[test]
    fn day10_simple_p1() {
        let mut input = SIMPLE.to_owned();
        input.sort();
        assert_eq!(differences(&input), (7, 5))
    }

    #[test]
    fn day10_simple_p2() {
        let mut input = SIMPLE.to_owned();
        input.sort();
        assert_eq!(chain_count(&input), 8)
    }
}

pub fn part_2(input: Input) -> color_eyre::Result<String> {
    input.sort();
    let result = chain_count(input);
    Ok(format!("Number of arangements: {}", result))
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Vec<u64>> {
    context.parse_lines(|s| Ok(s.parse()?))
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let mut input = parsing(context)?;
    context.execute(input.as_mut(), part_1, part_2)
}

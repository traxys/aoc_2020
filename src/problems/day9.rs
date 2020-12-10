use crate::DayContext;

type Input<'i> = &'i [u64];

fn is_valid(num: u64, prev: &[u64]) -> bool {
    for (i, a) in prev.iter().enumerate() {
        for b in &prev[(i + 1)..] {
            if a + b == num {
                return true;
            }
        }
    }
    return false;
}

fn first_invalid(input: Input) -> color_eyre::Result<u64> {
    input
        .windows(26)
        .find_map(|window| match window {
            [prev @ .., num] if !is_valid(*num, prev) => Some(num),
            _ => None,
        })
        .copied()
        .ok_or_else(|| color_eyre::eyre::eyre!("No invalid num found"))
}

pub fn part_1(input: Input) -> color_eyre::Result<String> {
    let invalid = first_invalid(input)?;
    Ok(format!("First invalid: {}", invalid))
}

struct GrowingWindows<'i, T> {
    current: std::ops::Range<usize>,
    slice: &'i [T],
}

impl<'i, T> GrowingWindows<'i, T> {
    fn new(slice: &'i [T]) -> Self {
        Self {
            slice,
            current: 0..1,
        }
    }
}

impl<'i, T> Iterator for GrowingWindows<'i, T> {
    type Item = &'i [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.end > self.slice.len() {
            None
        } else {
            let item = &self.slice[self.current.clone()];
            self.current.end += 1;
            Some(item)
        }
    }
}

pub fn part_2(input: Input) -> color_eyre::Result<String> {
    let invalid = first_invalid(input)?;

    for i in 0..input.len() {
        if let Some(set) = GrowingWindows::new(&input[i..])
            // At least 2 elements, skipping the only elem
            .skip(1)
            .map(|set| (set, set.iter().sum::<u64>()))
            .take_while(|&(_, i)| i <= invalid)
            .find(|&(_, i)| i == invalid)
            .map(|(set, _)| set)
        {
            let mut min = u64::MAX;
            let mut max = 0;
            for &elem in set {
                if elem > max {
                    max = elem;
                } else if elem < min {
                    min = elem
               }
            }

            return Ok(format!("Weakness is: {}", min + max));
        }
    }
    todo!()
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Vec<u64>> {
    context.parse_lines(|s| Ok(s.parse()?))
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input.as_ref(), part_1, part_2)
}

use crate::DayContext;

type Input = Timetable;

// P = X - (X % B)
// N = P + B
// W = N - X = P + B - X = X- (X % B) + B - X = B - (X % B)

pub fn part_1(tt: Input) -> color_eyre::Result<String> {
    let (bus, wait) = tt
        .cycles
        .iter()
        .filter_map(|bus| match bus {
            Bus::Ignored => None,
            Bus::Present(bus) => Some(bus),
        })
        .map(|&bus| {
            let wait_time = bus - (tt.current_time % bus);
            (bus, wait_time)
        })
        .min_by_key(|&(_, tm)| tm)
        .ok_or_else(|| color_eyre::eyre::eyre!("No element in the cycle"))?;

    Ok(format!("ID*wait time: {}", bus * wait))
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, m: i64) -> i64 {
    let (g, x, _) = egcd(x, m);
    if g == 1 {
        (x % m + m) % m
    } else {
        panic!("Input are coprime")
    }
}

fn chinese_remainder(residues: &[i64], moduli: &[i64]) -> i64 {
    let n: i64 = moduli.iter().product();

    let mut sum = 0;
    for (&mod_i, &b_i) in residues.iter().zip(moduli) {
        let p = n / b_i;
        sum += mod_i * mod_inv(p, b_i) * p;
    }

    sum % n
}

// find t such that
// for bus b_i we have  b_i - (t % b_i) = i
// -(t % b_i) = i - b_i
// t % b_i = b_i - i
// t = b_i - i (mod b_i)
// t = mod_i (mod b_i)

fn sequential_leaving(cycles: &[Bus]) -> i64 {
    let (moduli, residue): (Vec<_>, Vec<_>) = cycles
        .iter()
        .enumerate()
        .filter_map(|(i, &bus)| match bus {
            Bus::Ignored => None,
            Bus::Present(bus) => {
                let i = i as i64;
                let b_i = bus as i64;
                let mut mod_i = b_i - i;
                while mod_i < 0 {
                    mod_i += b_i;
                }
                Some((b_i, mod_i))
            }
        })
        .unzip();
    chinese_remainder(&residue, &moduli)
}

pub fn part_2(tt: Input) -> color_eyre::Result<String> {
    let solution = sequential_leaving(&tt.cycles);
    Ok(format!("Such timestamp is: {}", solution))
}

#[cfg(test)]
mod test {
    use super::{sequential_leaving, Bus};

    #[test]
    fn day13_pt2_simple() {
        let cycles = vec![
            Bus::Present(17),
            Bus::Ignored,
            Bus::Present(13),
            Bus::Present(19),
        ];

        assert_eq!(sequential_leaving(&cycles), 3417);
    }
}

#[derive(Debug)]
pub struct Timetable {
    current_time: u64,
    cycles: Vec<Bus>,
}

#[derive(Debug, Copy, Clone)]
enum Bus {
    Present(u64),
    Ignored,
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    let current_time = context.read_line()?.parse()?;
    let cycles: color_eyre::Result<_> = context
        .read_line()?
        .split(",")
        .map(|s| match s {
            "x" => Ok(Bus::Ignored),
            id => Ok(Bus::Present(id.parse()?)),
        })
        .collect();

    Ok(Timetable {
        current_time,
        cycles: cycles?,
    })
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}

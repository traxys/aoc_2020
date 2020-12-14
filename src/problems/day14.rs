use std::{collections::HashMap, str::FromStr};

use crate::DayContext;

type Input = Vec<Instr>;

pub fn part_1(input: Input) -> color_eyre::Result<String> {
    let mut computer = Computer::new();
    computer.execute(&input);

    Ok(format!("The sum of the memory is: {}", computer.sum()))
}

pub fn part_2(input: Input) -> color_eyre::Result<String> {
    let mut computer = Computer::new();
    computer.execute_v2(&input);

    Ok(format!("The sum of the memory is: {}", computer.sum()))
}

struct Computer {
    current_mask: Mask,
    values: HashMap<u64, u64>,
}

impl Computer {
    fn new() -> Self {
        Self {
            current_mask: Mask::nop(),
            values: HashMap::new(),
        }
    }

    fn execute(&mut self, instrs: &[Instr]) {
        for instr in instrs {
            match instr {
                Instr::Mask(m) => self.current_mask = *m,
                Instr::Set { address, value } => {
                    self.values.insert(*address, self.current_mask.set(*value));
                }
            }
        }
    }

    fn execute_v2(&mut self, instrs: &[Instr]) {
        for instr in instrs {
            match instr {
                Instr::Mask(m) => self.current_mask = *m,
                Instr::Set { address, value } => {
                    for add in self.current_mask.floating_adresses(*address) {
                        self.values.insert(add, *value);
                    }
                }
            }
        }
    }

    fn sum(&self) -> u64 {
        self.values.values().sum()
    }
}

#[derive(Clone, Copy)]
pub struct Mask {
    ignored: u64,
    set: u64,
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ignored = self.ignored << (64 - 36);
        let mut set = self.set << (64 - 36);

        for _ in 0..36 {
            if ignored & (1 << 63) > 0 {
                write!(f, "X")?;
            } else {
                if set & (1 << 63) > 0 {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
            }
            ignored <<= 1;
            set <<= 1;
        }

        Ok(())
    }
}

impl Mask {
    fn nop() -> Self {
        Self {
            ignored: !0,
            set: 0,
        }
    }

    fn set(&self, value: u64) -> u64 {
        let overwrite = (value & self.set) | self.set;
        let pass_through = value & self.ignored;
        overwrite | pass_through
    }

    fn floating_adresses(&self, address: u64) -> impl Iterator<Item = u64> + '_ {
        let applied = address | self.set;
        // The number of bits to replace
        (0..(1 << self.ignored.count_ones()))
            .map(move |i| self.floating_mask(i))
            .map(move |m| m.set(applied))
    }

    fn floating_mask(&self, mut replacement: u64) -> Mask {
        let mut current_floating = self.ignored;

        let mut ignored = 0;
        let mut set = 0;

        for i in 0..36 {
            if current_floating & 1 > 0 {
                set |= (replacement & 1) << i;
                replacement >>= 1;
            } else {
                ignored |= 1 << i;
            };
            current_floating >>= 1;
        }

        Mask { ignored, set }
    }
}

impl FromStr for Mask {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (set, ignored) =
            s.as_bytes()
                .iter()
                .rev()
                .enumerate()
                .fold((0, 0), |(set, ignored), (i, b)| match b {
                    b'X' => (set, ignored | 1 << i),
                    b'1' => (set | 1 << i, ignored),
                    b'0' => (set, ignored),
                    _ => unreachable!(),
                });

        Ok(Self { set, ignored })
    }
}

pub enum Instr {
    Mask(Mask),
    Set { address: u64, value: u64 },
}

#[cfg(test)]
mod test {
    use super::Instr;

    fn simple_input() -> Vec<Instr> {
        vec![
            Instr::Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap()),
            Instr::Set {
                address: 8,
                value: 11,
            },
            Instr::Set {
                address: 7,
                value: 101,
            },
            Instr::Set {
                address: 8,
                value: 0,
            },
        ]
    }

    #[test]
    fn day14_simple_p1() {
        let mut computer = super::Computer::new();
        computer.execute(&simple_input());
        assert_eq!(computer.sum(), 165)
    }

    #[test]
    fn day14_memory_possibilities_4() {
        let mask: super::Mask = "000000000000000000000000000000X1001X".parse().unwrap();
        let addresses: Vec<_> = mask.floating_adresses(42).collect();
        assert_eq!(addresses, vec![26, 27, 58, 59]);
    }

    #[test]
    fn day14_memory_possibilities_8() {
        let mask: super::Mask = "00000000000000000000000000000000X0XX".parse().unwrap();
        let addresses: Vec<_> = mask.floating_adresses(26).collect();
        assert_eq!(addresses, vec![16, 17, 18, 19, 24, 25, 26, 27]);
    }
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    context.parse_lines(|line| match line.strip_prefix("mask = ") {
        None => match line.strip_prefix("mem[") {
            None => color_eyre::eyre::bail!("Malformed line: {}", line),
            Some(mem_line) => {
                let (address, value) = crate::large_split_str_sep(mem_line, "] = ")
                    .ok_or_else(|| color_eyre::eyre::eyre!("Invalid line: {}", line))?;
                Ok(Instr::Set {
                    address: address.parse()?,
                    value: value.parse()?,
                })
            }
        },
        Some(mask) => Ok(Instr::Mask(mask.parse()?)),
    })
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}

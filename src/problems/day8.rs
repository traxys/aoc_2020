use std::collections::HashSet;

use crate::DayContext;

type Input<'i> = &'i [Instr];

#[derive(Clone, Copy)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}

#[derive(Clone, Copy)]
pub struct Instr {
    op: OpCode,
    arg: i64,
}

pub fn part_1(code: Input) -> color_eyre::Result<String> {
    let mut acc = 0;
    let mut ip = 0;
    let mut visited = HashSet::new();

    let final_value = loop {
        if visited.contains(&ip) {
            break acc;
        }
        visited.insert(ip);
        let instr = &code[ip];
        match instr.op {
            OpCode::Acc => {
                acc += instr.arg;
                ip += 1;
            }
            OpCode::Nop => {
                ip += 1;
            }
            OpCode::Jmp => {
                ip = (ip as i64 + instr.arg) as usize;
            }
        }
    };

    Ok(format!("Accumulator is at: {}", final_value))
}

pub fn part_2(code: Input) -> color_eyre::Result<String> {
    let mut patched = code.to_owned();
    for (index, instr) in code.iter().enumerate().filter(|(_, i)| match i.op {
        OpCode::Acc => false,
        _ => true,
    }) {
        patched[index].op = match instr.op {
            OpCode::Jmp => OpCode::Nop,
            OpCode::Nop => OpCode::Jmp,
            _ => unreachable!(),
        };

        let mut acc = 0;
        let mut ip = 0;
        let mut visited = HashSet::new();

        loop {
            if ip == patched.len() {
                return Ok(format!("Patched accumulator is: {}", acc))
            }

            if visited.contains(&ip) {
                break;
            }
            visited.insert(ip);
            let instr = &patched[ip];

            match instr.op {
                OpCode::Acc => {
                    acc += instr.arg;
                    ip += 1;
                }
                OpCode::Nop => {
                    ip += 1;
                }
                OpCode::Jmp => {
                    ip = (ip as i64 + instr.arg) as usize;
                }
            }
        }

        patched[index] = *instr;
    }

    color_eyre::eyre::bail!("Did not find a correct patch")
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Vec<Instr>> {
    context.parse_lines(|line| {
        let (opcode, arg) = crate::split_string_separator(line, ' ')
            .ok_or_else(|| color_eyre::eyre::eyre!("Malformed line: {}", line))?;
        let op = match opcode {
            "nop" => OpCode::Nop,
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            _ => color_eyre::eyre::bail!("No such opcode: {}", opcode),
        };

        Ok(Instr {
            op,
            arg: arg.parse()?,
        })
    })
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input.as_ref(), part_1, part_2)
}

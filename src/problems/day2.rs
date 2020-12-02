use crate::{split_bytes_separator, DayContext};
use bstr::{BStr, BString, ByteSlice};

pub fn part_1(passwords: &[(Policy, BString)]) -> color_eyre::Result<String> {
    let valid_count = passwords
        .iter()
        .filter_map(|(policy, password)| {
            if policy.is_old_valid(password.as_ref()) {
                Some(true)
            } else {
                None
            }
        })
        .count();
    Ok(format!("There are {} valid passwords", valid_count))
}

pub fn part_2(passwords: &[(Policy, BString)]) -> color_eyre::Result<String> {
    let valid_count = passwords
        .iter()
        .filter_map(|(policy, password)| {
            if policy.is_current_valid(password.as_ref()) {
                Some(true)
            } else {
                None
            }
        })
        .count();
    Ok(format!("There are {} valid passwords", valid_count))
}

pub struct Policy {
    a: usize,
    b: usize,
    letter: u8,
}

impl Policy {
    fn is_old_valid(&self, password: &BStr) -> bool {
        (self.a..(self.b + 1)).contains(&password.iter().filter(|&&l| l == self.letter).count())
    }

    fn is_current_valid(&self, password: &BStr) -> bool {
        let ascii_chars = password.as_bytes();
        (ascii_chars[self.a] == self.letter as u8) ^ (ascii_chars[self.b] == self.letter as u8)
    }

    fn parse(inp: &BStr) -> color_eyre::Result<Self> {
        let (range, letter) = split_bytes_separator(inp, b' ')
            .ok_or_else(|| color_eyre::eyre::eyre!("Missing ' '"))?;

        let (a, b) = split_bytes_separator(range, b'-')
            .ok_or_else(|| color_eyre::eyre::eyre!("Malformed range"))?;

        let letter = letter
            .get(0)
            .copied()
            .ok_or_else(|| color_eyre::eyre::eyre!("No letter after the range"))?;

        Ok(Policy {
            a: a.to_str()?.parse()?,
            b: b.to_str()?.parse()?,
            letter,
        })
    }
}

pub fn parsing(ctx: &mut DayContext) -> color_eyre::Result<Vec<(Policy, BString)>> {
    let lines = ctx.parse_byte_lines(|line| -> color_eyre::eyre::Result<_> {
        let (policy, password) = split_bytes_separator(line, b':')
            .ok_or_else(|| color_eyre::eyre::eyre!("Colud not split password at ':'"))?;
        Ok((Policy::parse(policy.as_ref())?, BString::from(password)))
    })?;
    Ok(lines)
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let passwords = parsing(context)?;
    context.execute(passwords.as_ref(), part_1, part_2)
}

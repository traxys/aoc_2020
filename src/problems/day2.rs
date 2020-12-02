use crate::{split_string_separator, DayContext};

pub fn part_1(passwords: &[(Policy, String)]) -> color_eyre::Result<String> {
    let valid_count = passwords
        .iter()
        .filter_map(|(policy, password)| {
            if policy.is_old_valid(&password) {
                Some(true)
            } else {
                None
            }
        })
        .count();
    Ok(format!("There are {} valid passwords", valid_count))
}

pub fn part_2(passwords: &[(Policy, String)]) -> color_eyre::Result<String> {
    let valid_count = passwords
        .iter()
        .filter_map(|(policy, password)| {
            if policy.is_current_valid(&password) {
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
    letter: char,
}

impl Policy {
    fn is_old_valid(&self, password: &str) -> bool {
        (self.a..(self.b + 1)).contains(&password.chars().filter(|&l| l == self.letter).count())
    }

    fn is_current_valid(&self, password: &str) -> bool {
        let ascii_chars = password.as_bytes();
        (ascii_chars[self.a] == self.letter as u8) ^ (ascii_chars[self.b] == self.letter as u8)
    }
}

impl std::str::FromStr for Policy {
    type Err = color_eyre::eyre::Error;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        let (range, letter) = split_string_separator(inp, ' ')
            .ok_or_else(|| color_eyre::eyre::eyre!("Missing ' '"))?;

        let (a, b) = split_string_separator(range, '-')
            .ok_or_else(|| color_eyre::eyre::eyre!("Malformed range"))?;

        let letter = letter
            .chars()
            .nth(0)
            .ok_or_else(|| color_eyre::eyre::eyre!("No letter after the range"))?;

        Ok(Policy {
            a: a.parse()?,
            b: b.parse()?,
            letter,
        })
    }
}

pub fn parsing(ctx: &mut DayContext) -> color_eyre::Result<Vec<(Policy, String)>> {
    let lines = ctx.parse_lines(|line| -> color_eyre::eyre::Result<_> {
        let (policy, password) = split_string_separator(line, ':')
            .ok_or_else(|| color_eyre::eyre::eyre!("Colud not split password at ':'"))?;
        Ok((policy.parse()?, password.to_owned()))
    })?;
    Ok(lines)
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let passwords = parsing(context)?;
    context.execute(passwords.as_ref(), part_1, part_2)
}

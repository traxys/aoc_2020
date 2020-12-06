use crate::DayContext;
use bstr::ByteSlice;

type Input<'i> = &'i [Vec<u32>];

pub fn part_1(groups: Input) -> color_eyre::Result<String> {
    let answers: u32 = groups
        .iter()
        .map(|group| group.iter().fold(0, |group, person| group | person))
        .map(u32::count_ones)
        .sum();
    Ok(format!("Number of yes (anyone): {}", answers))
}

pub fn part_2(groups: Input) -> color_eyre::Result<String> {
    let answers: u32 = groups
        .iter()
        .map(|group| group.iter().fold(!0, |group, person| group & person))
        .map(u32::count_ones)
        .sum();
    Ok(format!("Number of yes (everyone): {}", answers))
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Vec<Vec<u32>>> {
    let mut groups = vec![Vec::new()];

    context.accumulate_byte_lines(|_, line| {
        if line.len() == 0 {
            groups.push(Vec::new());
        } else {
            let group = groups.last_mut().unwrap();
            let mut person = 0;
            for letter in line.as_bytes() {
                let pos = letter - b'a';
                person |= 1 << pos;
            }
            group.push(person)
        }

        Ok(())
    })?;

    Ok(groups)
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input.as_ref(), part_1, part_2)
}

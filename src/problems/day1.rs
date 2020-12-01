use crate::DayContext;

pub fn part_1(lines: &[u64]) -> color_eyre::Result<String> {
    let mut expense = &lines[..];
    while expense.len() > 2 {
        let rest = &expense[1..];
        let value = expense[0];
        for other in rest {
            if other + value == 2020 {
                return Ok(format!(
                    "Found {} + {} = 2020; result is {}",
                    value,
                    other,
                    other * value
                ));
            }
        }
        expense = rest;
    }
    color_eyre::eyre::bail!("Did not find an answer")
}

pub fn part_2(lines: &[u64]) -> color_eyre::Result<String> {
    let mut expense = &lines[..];
    while expense.len() > 3 {
        let value = expense[0];

        let seconds = &expense[1..];
        let thirds = &expense[2..];
        for second in seconds {
            for third in thirds {
                if value + second + third == 2020 {
                    return Ok(format!(
                        "Found {} + {} + {} = 2020; result is {}",
                        value,
                        second,
                        third,
                        value * second * third
                    ));
                }
            }
        }
        expense = seconds;
    }
    color_eyre::eyre::bail!("Did not find an answer")
}

pub fn parsing(ctx: &mut DayContext) -> color_eyre::Result<Vec<u64>> {
    let lines = ctx.parse_lines(|x| -> Result<u64, _> { x.parse() })?;
    Ok(lines)
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let lines = parsing(context)?;
    context.execute(lines.as_ref(), part_1, part_2)
}

const DAY: &str = "day1";
use aoc_2020::read_input;

fn part_1(lines: Vec<u64>) -> color_eyre::Result<()> {
    let mut expense = &lines[..];
    while expense.len() > 2 {
        let rest = &expense[1..];
        let value = expense[0];
        for other in rest {
            if other + value == 2020 {
                println!(
                    "Found {} + {} = 2020; result is {}",
                    value,
                    other,
                    other * value
                );
                return Ok(());
            }
        }
        expense = rest;
    }
    color_eyre::eyre::bail!("Did not find an answer")
}

fn part_2(lines: Vec<u64>) -> color_eyre::Result<()> {
    let mut expense = &lines[..];
    while expense.len() > 3 {
        let value = expense[0];

        let seconds = &expense[1..];
        let thirds = &expense[2..];
        for second in seconds {
            for third in thirds {
                if value + second + third == 2020 {
                    println!(
                        "Found {} + {} + {} = 2020; result is {}",
                        value,
                        second,
                        third,
                        value * second * third
                    );
                    return Ok(());
                }
            }
        }
        expense = seconds;
    }
    color_eyre::eyre::bail!("Did not find an answer")
}

fn main() -> color_eyre::Result<()> {
    let input = read_input(DAY)?;
    let lines = aoc_2020::read_all_lines(input, |x: String| -> Result<u64, _> { x.parse() })?;
    aoc_2020::execute_day(lines, part_1, part_2)
}

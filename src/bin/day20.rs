use aoc_2020::{problems::day20::execute, DayContext};

fn main() -> color_eyre::Result<()> {
    let mut context = DayContext::load()?;
    execute(&mut context)?;
    context.report_timings();
    Ok(())
}
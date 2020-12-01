use color_eyre::eyre::Context;
use std::io::BufRead;
use structopt::StructOpt;

const INPUTS: &str = "inputs";

enum Part {
    One,
    Two,
}

impl std::str::FromStr for Part {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            _ => color_eyre::eyre::bail!("Invalid part number"),
        }
    }
}

#[derive(StructOpt)]
struct Args {
    #[structopt(short, long, default_value = "1", possible_values = &["1", "2"])]
    part: Part,
}

pub fn execute_day<I, P1: FnOnce(I) -> color_eyre::Result<()>, P2: FnOnce(I) -> color_eyre::Result<()>>(input: I, part1: P1, part2: P2) -> color_eyre::Result<()> {
    let args = Args::from_args();

    match args.part {
        Part::One => part1(input),
        Part::Two => part2(input),
    }
}

pub fn read_input(day: &str) -> color_eyre::Result<impl BufRead> {
    let mut path: std::path::PathBuf = INPUTS.into();
    path.push(day);
    let input = std::fs::File::open(path)
        .with_context(|| format!("could not open input file at {}/{}", INPUTS, day))?;
    Ok(std::io::BufReader::new(input))
}

pub fn read_all_lines<B: BufRead, I, E: Send + Sync + std::error::Error + 'static, F: FnMut(String) -> Result<I, E>>(
    input: B,
    mut parser: F,
) -> color_eyre::Result<Vec<I>> {
    input
        .lines()
        .map(move |line| -> color_eyre::Result<I> {
            match line {
                Ok(x) => parser(x).with_context(|| "Error parsing line"),
                Err(e) => Err(e).with_context(|| "Error reading line"),
            }
        })
        .collect()
}

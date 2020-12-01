use color_eyre::eyre::Context;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use structopt::StructOpt;

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

pub struct DayContext {
    part: Part,
    input_file: BufReader<File>,
}

impl DayContext {
    pub fn load() -> color_eyre::Result<Self> {
        let args = Args::from_args();
        let input_file = File::open(&args.input)
            .with_context(|| format!("Could not open input: {:?}", args.input))?;
        let input_file = BufReader::new(input_file);

        Ok(Self {
            input_file,
            part: args.part,
        })
    }

    pub fn execute<I, P1, P2>(&self, input: I, part1: P1, part2: P2) -> color_eyre::Result<()>
    where
        P1: FnOnce(I) -> color_eyre::Result<()>,
        P2: FnOnce(I) -> color_eyre::Result<()>,
    {
        match self.part {
            Part::One => part1(input),
            Part::Two => part2(input),
        }
    }

    pub fn parse_lines<
        I,
        E: Send + Sync + std::error::Error + 'static,
        F: FnMut(&str) -> Result<I, E>,
    >(
        &mut self,
        mut parser: F,
    ) -> color_eyre::Result<Vec<I>> {
        let mut result = Vec::new();

        let mut buf = String::new();
        loop {
            buf.clear();
            match self
                .input_file
                .read_line(&mut buf)
                .with_context(|| "Could not read line in the input file")?
            {
                0 => break,
                _ => {
                    if buf.ends_with('\n') {
                        buf.pop();
                        if buf.ends_with('\r') {
                            buf.pop();
                        }
                    }
                    result.push(
                        parser(&buf)
                            .with_context(|| format!("Could not parse the line {}", buf))?,
                    );
                }
            }
        }

        Ok(result)
    }
}

#[derive(StructOpt)]
struct Args {
    #[structopt(short, long, default_value = "1", possible_values = &["1", "2"])]
    part: Part,
    #[structopt(short, long)]
    input: PathBuf,
}

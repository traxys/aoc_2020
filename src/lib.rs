use color_eyre::eyre::Context;
use std::{fmt::Display, fs::File, io::{BufRead, BufReader}, path::PathBuf, time::Duration, time::Instant};
use structopt::StructOpt;

pub mod problems;

enum Part {
    One,
    Two,
}

#[inline]
pub fn open_input(day: u8, mut folder: PathBuf) -> color_eyre::Result<File> {
    folder.push(&format!("day{}", day));
    File::open(folder).map_err(Into::into)
}

pub struct DayContext {
    part: Part,
    input_file: BufReader<File>,
    timing: Timing,
    report: bool,
}

pub struct Timing {
    pub parsing: Option<Duration>,
    pub execution: Option<Duration>,
}

impl Timing {
    fn new() -> Self {
        Self {
            parsing: None,
            execution: None,
        }
    }
}

fn format_time(dur: Duration) -> String {
    let elapsed = dur.as_nanos() as f64;

    let secs_ns = (10.0f64).powi(9);
    let ms_ns = (10.0f64).powi(6);
    let us_ns = (10.0f64).powi(3);
    if elapsed > secs_ns {
        format!("{:.2}s", elapsed / secs_ns)
    } else if elapsed > ms_ns {
        format!("{:.2}ms", elapsed / ms_ns)
    } else if elapsed > us_ns {
        format!("{:.2}us", elapsed / us_ns)
    } else {
        format!("{:.2}ns", elapsed)
    }
}

impl DayContext {
    pub fn timings(&self) -> &Timing {
        &self.timing
    }

    pub fn report_timings(&self) {
        println!("");
        println!("TIMINGS");
        if let Some(parsing) = self.timing.parsing {
            println!("  - Time taken for parsing: {}", format_time(parsing))
        }

        if let Some(execution) = self.timing.execution {
            println!("  - Time taken for execution: {}", format_time(execution))
        }
    }

    pub fn load() -> color_eyre::Result<Self> {
        let args = Args::from_args();
        let input_file = File::open(&args.input)
            .with_context(|| format!("Could not open input: {:?}", args.input))?;
        let input_file = BufReader::new(input_file);

        Ok(Self {
            timing: Timing::new(),
            input_file,
            part: args.part,
            report: true,
        })
    }

    fn new(file: File, part: Part) -> Self {
        Self {
            input_file: BufReader::new(file),
            timing: Timing::new(),
            part,
            report: false,
        }
    }

    #[inline]
    pub fn new_part1(file: File) -> Self {
        Self::new(file, Part::One)
    }
    #[inline]
    pub fn new_part2(file: File) -> Self {
        Self::new(file, Part::Two)
    }

    pub fn execute<I, R, P1, P2>(&mut self, input: I, part1: P1, part2: P2) -> color_eyre::Result<()>
    where
        R: Display,
        P1: FnOnce(I) -> color_eyre::Result<R>,
        P2: FnOnce(I) -> color_eyre::Result<R>,
    {
        let start = Instant::now();
        let res = match self.part {
            Part::One => part1(input)?,
            Part::Two => part2(input)?,
        };
        if self.timing.execution.is_none() {
            self.timing.execution = Some(start.elapsed());
        }

        if self.report {
            println!("{}", res);
        }

        Ok(())
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
        let start = Instant::now();
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

        if self.timing.parsing.is_none() {
            self.timing.parsing = Some(start.elapsed());
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

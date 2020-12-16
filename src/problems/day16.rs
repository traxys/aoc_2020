use crate::DayContext;
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

type Input = Scan;

pub fn part_1(scan: Input) -> color_eyre::Result<String> {
    let error_rate: u64 = scan
        .scanned
        .iter()
        .filter_map(|ticket| {
            let invalid: Vec<_> = ticket
                .iter()
                .filter(|&&value| scan.fields.iter().all(|field| !field.in_range(value)))
                .collect();
            if invalid.len() == 0 {
                None
            } else {
                Some(invalid)
            }
        })
        .map(|invalid| invalid.into_iter().sum::<u64>())
        .sum();

    Ok(format!("Error rate is: {}", error_rate))
}

fn associate_field<'f, 't>(
    fields: &'t HashSet<&'f Field>,
    tickets: &'t [&Vec<u64>],
    field_number: usize,
) -> Option<&'f Field> {
    // This creates a clone, but with &Field
    let mut remaining_fields: HashSet<_> = fields.iter().collect();

    for field in fields {
        if tickets
            .iter()
            .any(|ticket| !field.in_range(ticket[field_number]))
        {
            remaining_fields.remove(field);
        }
    }

    if remaining_fields.len() > 1 {
        return None;
    }

    remaining_fields.into_iter().nth(0).copied()
}

pub fn part_2(scan: Input) -> color_eyre::Result<String> {
    let valid_tickets: Vec<_> = scan
        .scanned
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|&value| scan.fields.iter().any(|field| field.in_range(value)))
        })
        .collect();

    let mut remaining_fields: HashSet<_> = scan.fields.iter().collect();
    let mut fields = HashMap::new();

    while !remaining_fields.is_empty() {
        for field_number in 0..scan.own_ticket.len() {
            let field = associate_field(&remaining_fields, &valid_tickets, field_number);
            if let Some(field) = field {
                remaining_fields.remove(field);
                fields.insert(field, field_number);
            }
        }
    }

    let field_product: u64 = fields
        .iter()
        .filter(|(field, _)| field.name.starts_with("departure"))
        .map(|(_, &number)| scan.own_ticket[number])
        .product();

    Ok(format!("The product of the fields is: {}", field_product))
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Field {
    name: String,
    range_a: Range<u64>,
    range_b: Range<u64>,
}

impl Field {
    fn in_range(&self, number: u64) -> bool {
        self.range_a.contains(&number) || self.range_b.contains(&number)
    }
}

pub struct Scan {
    fields: Vec<Field>,
    own_ticket: Vec<u64>,
    scanned: Vec<Vec<u64>>,
}

fn parse_range(range: &str) -> color_eyre::Result<Range<u64>> {
    let (start, end) = crate::split_string_separator(range, '-')
        .ok_or_else(|| color_eyre::eyre::eyre!("Malformed range: {}", range))?;
    Ok((start.parse()?)..(end.parse::<u64>()? + 1))
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    let mut fields = Vec::new();
    let mut done_fields = false;
    let mut own_ticket = None;
    let mut scanned = Vec::new();

    context.accumulate_str_lines(|_, line| {
        if !done_fields {
            if line == "" {
                done_fields = true;
                return Ok(());
            }
            let (name, ranges) = crate::split_string_separator(line, ':')
                .ok_or_else(|| color_eyre::eyre::eyre!("Malformed field: {}", line))?;
            let (range_a, range_b) = crate::large_split_str_sep(ranges, " or ")
                .ok_or_else(|| color_eyre::eyre::eyre!("Malformed ranges: {}", ranges))?;
            let field = Field {
                name: name.to_owned(),
                range_a: parse_range(range_a.trim_start())?,
                range_b: parse_range(range_b)?,
            };
            fields.push(field);
        } else {
            if own_ticket.is_none() {
                if line == "your ticket:" {
                    return Ok(());
                } else {
                    own_ticket = Some(
                        line.split(',')
                            .map(|x| x.parse())
                            .collect::<Result<_, _>>()?,
                    );
                }
            } else {
                if !(line == "nearby tickets:" || line == "") {
                    scanned.push(
                        line.split(',')
                            .map(|x| x.parse())
                            .collect::<Result<_, _>>()?,
                    )
                }
            }
        };
        Ok(())
    })?;

    Ok(Scan {
        fields,
        scanned,
        own_ticket: own_ticket
            .ok_or_else(|| color_eyre::eyre::eyre!("No own ticket was parsed"))?,
    })
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}

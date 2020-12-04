use crate::DayContext;

pub fn part_1(passports: &[Option<Passport>]) -> color_eyre::Result<String> {
    let valid_count = passports
        .iter()
        .filter(|passport| passport.is_some())
        .count();

    Ok(format!("Valid passports: {}", valid_count))
}

fn parse_valid(passport: &Passport) -> color_eyre::Result<bool> {
    let byr: u64 = passport.byr.parse()?;
    if byr < 1920 || byr > 2020 {
        return Ok(false);
    }

    let iyr: u64 = passport.iyr.parse()?;
    if iyr < 2010 || iyr > 2020 {
        return Ok(false);
    }

    let eyr: u64 = passport.eyr.parse()?;
    if eyr < 2020 || eyr > 2030 {
        return Ok(false);
    }

    match passport.hgt.strip_suffix("cm") {
        Some(hgt) => {
            let hgt: u64 = hgt.parse()?;
            if hgt < 150 || hgt > 193 {
                return Ok(false);
            }
        }
        None => match passport.hgt.strip_suffix("in") {
            Some(hgt) => {
                let hgt: u64 = hgt.parse()?;
                if hgt < 59 || hgt > 76 {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        },
    }

    match passport.hcl.strip_prefix('#') {
        Some(color) if color.len() == 6 => {
            if color
                .chars()
                .any(|c| (c < '0' || c > '9') && (c < 'a' || c > 'f'))
            {
                return Ok(false);
            }
        }
        _ => return Ok(false),
    }

    match passport.ecl.as_str() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
        _ => return Ok(false),
    }

    if passport.pid.len() != 9 {
        return Ok(false);
    }
    passport.pid.parse::<u64>()?;

    Ok(true)
}

pub fn part_2(passports: &[Option<Passport>]) -> color_eyre::Result<String> {
    let valid_count = passports
        .into_iter()
        .filter_map(|x| x.as_ref())
        .filter(|passport| match parse_valid(passport) {
            Err(_) => false,
            Ok(valid) => valid,
        })
        .count();

    Ok(format!("Valid passports: {}", valid_count))
}

#[allow(dead_code)]
pub struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

#[derive(Default)]
struct PassportBuilder {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl PassportBuilder {
    fn add_category(&mut self, category: &str, value: String) {
        match category {
            "byr" => self.byr = Some(value),
            "iyr" => self.iyr = Some(value),
            "eyr" => self.eyr = Some(value),
            "hgt" => self.hgt = Some(value),
            "hcl" => self.hcl = Some(value),
            "ecl" => self.ecl = Some(value),
            "pid" => self.pid = Some(value),
            "cid" => self.cid = Some(value),
            ukw => eprintln!("Unknown category: {}", ukw),
        }
    }

    fn build(self) -> Option<Passport> {
        Some(Passport {
            byr: self.byr?,
            iyr: self.iyr?,
            eyr: self.eyr?,
            hgt: self.hgt?,
            hcl: self.hcl?,
            ecl: self.ecl?,
            pid: self.pid?,
            cid: self.cid,
        })
    }
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let mut passports = Vec::new();
    let mut current_passport = PassportBuilder::default();

    context.accumulate_str_lines(|_, line| {
        if line.len() == 0 {
            let mut builder = Default::default();
            std::mem::swap(&mut builder, &mut current_passport);

            passports.push(builder.build());
        } else {
            for word in line.split_ascii_whitespace() {
                let (category, value) = crate::split_string_separator(word, ':')
                    .ok_or_else(|| color_eyre::eyre::eyre!("Malformed category: {}", word))?;

                current_passport.add_category(category, value.to_owned())
            }
        }

        Ok(())
    })?;
    passports.push(current_passport.build());

    context.execute(passports.as_ref(), part_1, part_2)
}

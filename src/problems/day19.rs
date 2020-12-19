use crate::DayContext;
use std::collections::{HashMap, HashSet};

type Input = (Rules, Vec<String>);

#[derive(Debug)]
enum Rule {
    Letter(char),
    Sequence(Vec<u64>),
}

#[derive(Debug)]
pub struct Rules {
    rules: HashMap<u64, Vec<Rule>>,
}

impl Rules {
    fn matches(&self, input: &str) -> bool {
        match self.munch(input, 0) {
            None => false,
            Some(s) => s.is_empty(),
        }
    }

    fn munch<'i>(&self, input: &'i str, rule: u64) -> Option<&'i str> {
        match self.rules.get(&rule) {
            None => panic!("Unknown rule reached"),
            Some(rules) => {
                'outer: for rule in rules {
                    match rule {
                        Rule::Letter(l) => {
                            let striped = input.strip_prefix(|c| c == *l);
                            if striped.is_some() {
                                return striped;
                            }
                        }
                        Rule::Sequence(s) => {
                            let mut inp = input;
                            for r in s {
                                match self.munch(inp, *r) {
                                    None => continue 'outer,
                                    Some(out) => inp = out,
                                }
                            }
                            return Some(inp);
                        }
                    }
                }
                None
            }
        }
    }

    fn looping_matches(
        &self,
        input: &str,
        rule_42: &HashSet<String>,
        rule_31: &HashSet<String>,
    ) -> bool {
        let ret = self.looping_munch(&vec![input], 0, rule_42, rule_31);
        !ret.is_empty() && ret.iter().any(|s| s.len() == 0)
    }

    fn looping_munch<'i>(
        &self,
        inputs: &[&'i str],
        rule: u64,
        rule_42: &HashSet<String>,
        rule_31: &HashSet<String>,
    ) -> Vec<&'i str> {
        let mut remains = Vec::new();
        if inputs.is_empty() {
            return remains;
        }

        if rule == 8 {
            let mut rem = inputs.to_owned();
            loop {
                let new_rem: Vec<_> = rem
                    .iter()
                    .map(|input| strip_set(input, rule_42))
                    .flatten()
                    .collect();
                if new_rem.is_empty() {
                    break;
                }

                remains.append(&mut new_rem.clone());
                rem = new_rem;
            }
        } else if rule == 11 {
            let len = rule_31.iter().nth(0).unwrap().len() + rule_42.iter().nth(0).unwrap().len();
            let max_input_len = inputs.iter().map(|s| s.len()).max().unwrap();
            let mut k = 1;
            while k * len <= max_input_len {
                for input in inputs {
                    remains.extend(
                        repeat_strip(input, rule_42, k)
                            .into_iter()
                            .map(|input| repeat_strip(input, rule_31, k).into_iter())
                            .flatten(),
                    )
                }
                k += 1;
            }
        } else {
            for input in inputs {
                let rules = self.rules.get(&rule).unwrap();
                for rule in rules {
                    match rule {
                        Rule::Letter(l) => {
                            let striped = input.strip_prefix(|c| c == *l);
                            if let Some(s) = striped {
                                remains.push(s);
                            }
                        }
                        Rule::Sequence(s) => {
                            let mut rem = vec![*input];
                            for r in s {
                                rem = self.looping_munch(&rem, *r, rule_42, rule_31);
                            }
                            remains = rem;
                        }
                    }
                }
            }
        }

        remains
    }

    fn productions(&self, rule: u64) -> HashSet<String> {
        let parts = self.rules.get(&rule).unwrap();

        parts
            .iter()
            .map(|part| match part {
                Rule::Letter(s) => {
                    let mut set = HashSet::new();
                    set.insert(format!("{}", s));
                    set
                }
                Rule::Sequence(s) => s.iter().copied().map(|r| self.productions(r)).fold(
                    {
                        let mut set = HashSet::new();
                        set.insert(String::new());
                        set
                    },
                    |current, seq| {
                        current
                            .into_iter()
                            .map(|prefix| seq.iter().map(move |suffix| prefix.to_owned() + suffix))
                            .flatten()
                            .collect()
                    },
                ),
            })
            .fold(HashSet::new(), |mut current, part| {
                current.extend(part);
                current
            })
    }
}

fn repeat_strip<'i: 's, 's>(
    input: &'i str,
    prefix_set: &'s HashSet<String>,
    amount: usize,
) -> Vec<&'i str> {
    let mut stripped = vec![input];

    for _ in 0..amount {
        stripped = stripped
            .into_iter()
            .map(|input| strip_set(input, prefix_set))
            .flatten()
            .collect();
    }

    stripped
}

fn strip_set<'i: 's, 's>(
    input: &'i str,
    prefix_set: &'s HashSet<String>,
) -> impl Iterator<Item = &'i str> + 's {
    prefix_set
        .iter()
        .filter_map(move |prefix| match input.strip_prefix(prefix) {
            Some(x) => Some(x),
            None => None,
        })
}

pub fn part_1((rules, messages): Input) -> color_eyre::Result<String> {
    let matching = messages.iter().filter(|msg| rules.matches(msg)).count();
    Ok(format!("Matching messages: {}", matching))
}

pub fn part_2((rules, messages): Input) -> color_eyre::Result<String> {
    let rule_42 = rules.productions(42);
    let rule_31 = rules.productions(31);

    let matching = messages
        .iter()
        .filter(|msg| rules.looping_matches(msg, &rule_42, &rule_31))
        .count();
    Ok(format!("Matching messages: {}", matching))
}

#[cfg(test)]
mod test {
    use super::{Input, Rules};
    use std::collections::HashMap;

    fn load_example() -> Input {
        let mut rules = HashMap::new();
        let mut rules_done = false;
        let mut messages = Vec::new();

        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"#;

        input.lines().for_each(|line| {
            super::parse_rules_messages(&mut rules, &mut rules_done, &mut messages, line).unwrap();
        });

        (Rules { rules }, messages)
    }

    #[test]
    fn test_loopy() {
        let (rules, messages) = load_example();

        let rule_42 = rules.productions(42);
        let rule_31 = rules.productions(31);

        let matching: Vec<_> = messages
            .iter()
            .filter(|msg| rules.looping_matches(msg, &rule_42, &rule_31))
            .collect();
        assert_eq!(
            matching,
            vec![
                "bbabbbbaabaabba",
                "babbbbaabbbbbabbbbbbaabaaabaaa",
                "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
                "bbbbbbbaaaabbbbaaabbabaaa",
                "bbbababbbbaaaaaaaabbababaaababaabab",
                "ababaaaaaabaaab",
                "ababaaaaabbbaba",
                "baabbaaaabbaaaababbaababb",
                "abbbbabbbbaaaababbbbbbaaaababb",
                "aaaaabbaabaaaaababaa",
                "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
                "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
            ]
        );
    }
}

fn parse_rules_messages(
    rules: &mut HashMap<u64, Vec<Rule>>,
    rules_done: &mut bool,
    messages: &mut Vec<String>,
    line: &str,
) -> color_eyre::Result<()> {
    if !*rules_done {
        if !line.is_empty() {
            let (number, body) = crate::split_string_separator(line, ':')
                .ok_or_else(|| color_eyre::eyre::eyre!("Malformed rule: {}", line))?;
            let parts = body
                .split("|")
                .map(|part| -> color_eyre::Result<_> {
                    if part.starts_with(" \"") {
                        Ok(Rule::Letter(
                            part.trim_start_matches(" \"").chars().nth(0).unwrap(),
                        ))
                    } else {
                        let sequence = part
                            .split_whitespace()
                            .map(str::parse)
                            .collect::<Result<_, _>>()?;
                        Ok(Rule::Sequence(sequence))
                    }
                })
                .collect::<Result<_, _>>()?;
            rules.insert(number.parse()?, parts);
        } else {
            *rules_done = true;
        }
    } else {
        messages.push(line.to_owned())
    }

    Ok(())
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    let mut rules = HashMap::new();
    let mut rules_done = false;
    let mut messages = Vec::new();

    context.accumulate_str_lines(|_, line| {
        parse_rules_messages(&mut rules, &mut rules_done, &mut messages, line)
    })?;

    Ok((Rules { rules }, messages))
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}

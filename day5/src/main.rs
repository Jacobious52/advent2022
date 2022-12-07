use std::collections::{HashMap, VecDeque};

use chumsky::prelude::*;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};

#[derive(Debug, Clone)]
struct Crates {
    stacks: HashMap<usize, VecDeque<char>>,
}

impl Crates {
    fn new(buf: Vec<String>) -> Self {
        let mut stacks = HashMap::new();
        buf[0]
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .for_each(|i| {
                for row in &buf[1..] {
                    let letter = row.chars().nth(1 + (i - 1) * 4).unwrap();
                    if letter.is_whitespace() {
                        continue;
                    }
                    stacks
                        .entry(i)
                        .and_modify(|v: &mut VecDeque<char>| v.push_back(letter))
                        .or_insert_with(|| vec![letter].into());
                }
            });

        Self { stacks }
    }
}

#[derive(Debug, Clone)]
struct Instr {
    count: usize,
    from: usize,
    to: usize,
}

fn parser() -> impl Parser<char, (Crates, Vec<Instr>), Error = Simple<char>> {
    let number = text::int(10)
        .map(|s: String| s.parse::<usize>().unwrap())
        .padded();

    let instrs = just("move")
        .then(number.clone())
        .then_ignore(just("from"))
        .then(number.clone())
        .then_ignore(just("to"))
        .then(number.clone())
        .map(|(((_, count), from), to)| Instr { count, from, to })
        .repeated()
        .at_least(1);

    take_until(instrs)
        .map(|(b, v)| {
            let buf = b
                .iter()
                .collect::<String>()
                .lines()
                .map(ToOwned::to_owned)
                .rev()
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>();
            (Crates::new(buf), v)
        })
        .then_ignore(end())
}

fn execute_part1(mut crates: Crates, instrs: Vec<Instr>) {
    for instr in instrs {
        for _ in 0usize..instr.count {
            let item = crates
                .stacks
                .get_mut(&instr.from)
                .unwrap()
                .pop_back()
                .unwrap();
            crates.stacks.get_mut(&instr.to).unwrap().push_back(item);
        }
    }

    let part1 = (1usize..=crates.stacks.len())
        .map(|i| crates.stacks.get(&i).unwrap().back().unwrap())
        .collect::<String>();

    println!("Part1 = {:?}", part1);
}

fn execute_part2(mut crates: Crates, instrs: Vec<Instr>) {
    for instr in instrs {
        let mut arm = VecDeque::new();

        for _ in 0usize..instr.count {
            let item = crates
                .stacks
                .get_mut(&instr.from)
                .unwrap()
                .pop_back()
                .unwrap();
            arm.push_back(item);
        }

        while !arm.is_empty() {
            let item = arm.pop_back().unwrap();
            crates.stacks.get_mut(&instr.to).unwrap().push_back(item);
        }
    }

    let part1 = (1usize..=crates.stacks.len())
        .map(|i| crates.stacks.get(&i).unwrap().back().unwrap())
        .collect::<String>();

    println!("Part2 = {:?}", part1);
}

fn main() {
    let input = include_str!("../input.txt");

    match parser().parse(input) {
        Ok((crates, instr)) => {
            execute_part1(crates.clone(), instr.clone());
            execute_part2(crates, instr);
        }
        Err(errs) => {
            errs.into_iter()
                .map(|e| e.map(|c| c.to_string()))
                .for_each(|e| {
                    let report = Report::build(ReportKind::Error, (), e.span().start);

                    let report = match e.reason() {
                        chumsky::error::SimpleReason::Unexpected => report
                            .with_message(format!(
                                "Unexpected token in input, expected {}",
                                e.expected()
                                    .map(|expected| match expected {
                                        Some(expected) => expected.to_string(),
                                        None => "end of input".to_string(),
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ))
                            .with_label(
                                Label::new(e.span())
                                    .with_message(format!(
                                        "Unexpected token {}",
                                        e.found()
                                            .unwrap_or(&"end of file".to_string())
                                            .fg(Color::Red)
                                    ))
                                    .with_color(Color::Red),
                            ),
                        _ => report.with_message("other error"),
                    };

                    report.finish().print(Source::from(&input)).unwrap()
                });
        }
    }
}

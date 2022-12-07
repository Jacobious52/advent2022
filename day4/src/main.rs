use std::ops::RangeInclusive;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::prelude::*;

#[derive(Debug, Clone)]
struct Assignment {
    left: RangeInclusive<i32>,
    right: RangeInclusive<i32>,
}

// lets try learn some combinator parsing
fn parser() -> impl Parser<char, Vec<Assignment>, Error = Simple<char>> {
    let parse_pair = text::int(10)
        .then(just('-').then(text::int(10)))
        .map(|(x, (_, y))| (x.parse().unwrap()..=y.parse().unwrap()))
        .padded();

    let parse_assignment = parse_pair
        .then(just(','))
        .then(parse_pair)
        .map(|((a, _), b)| Assignment { left: a, right: b });

    parse_assignment.repeated().then_ignore(end())
}

fn range_contains(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.contains(&b.start()) && a.contains(&b.end())
}

fn range_overlaps(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.contains(&b.start()) || a.contains(&b.end())
}

fn execute(assignments: Vec<Assignment>) {
    let mut part1 = 0;
    let mut part2 = 0;
    for a in assignments {
        let contains = range_contains(&a.left, &a.right) || range_contains(&a.right, &a.left);
        let overlaps = range_overlaps(&a.left, &a.right) || range_contains(&a.right, &a.left);
        if contains {
            part1 += 1;
        }
        if overlaps {
            part2 += 1;
        }
    }
    println!("Part1 = {}", part1);
    println!("Part1 = {}", part2);
}

fn main() {
    let input = include_str!("../input.txt");

    match parser().parse(input) {
        Ok(assignments) => execute(assignments),
        //try out some pretty error reporting
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

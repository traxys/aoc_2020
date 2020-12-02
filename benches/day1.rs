use aoc_2020::{problems::day1, DayContext};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = aoc_2020::open_input(1, "inputs".into()).expect("Could not read input");
    let mut context = DayContext::new_part1(input);
    let input = day1::parsing(&mut context).expect("parsing failed");

    c.bench_function("day1/part1", |b| b.iter(|| day1::part_1(&input)));
    c.bench_function("day1/part2", |b| b.iter(|| day1::part_2(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

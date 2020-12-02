use aoc_2020::{problems::day2, DayContext};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = aoc_2020::open_input(2, "inputs".into()).expect("Could not read input");
    let mut context = DayContext::new_part1(input);
    let input = day2::parsing(&mut context).expect("parsing failed");

    c.bench_function("day2/part1", |b| b.iter(|| day2::part_1(&input)));
    c.bench_function("day2/part2", |b| b.iter(|| day2::part_2(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use reinhardt_utils::text;
use std::hint::black_box;

fn capfirst_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("capfirst");

    let inputs = vec![
        ("short", "hello"),
        ("medium", "hello world from the benchmark"),
        ("long", "hello world from the benchmark testing performance of capfirst function with a longer string"),
        ("unicode", "hello 世界 мир"),
    ];

    for (name, input) in inputs.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, &s| {
            b.iter(|| text::capfirst(black_box(s)));
        });
    }

    group.finish();
}

fn title_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("title");

    let inputs = vec![
        ("short", "hello world"),
        ("medium", "hello world from the benchmark"),
        (
            "long",
            "hello world from the benchmark testing performance of title function",
        ),
        ("mixed", "hElLo WoRlD fRoM tHe BeNcHmArK"),
    ];

    for (name, input) in inputs.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, &s| {
            b.iter(|| text::title(black_box(s)));
        });
    }

    group.finish();
}

fn pluralize_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("pluralize");

    let words = vec![
        ("short", "cat"),
        ("medium", "category"),
        ("long", "internationalization"),
    ];

    for (name, word) in words.iter() {
        group.bench_with_input(BenchmarkId::new("singular", name), word, |b, &w| {
            b.iter(|| text::pluralize(black_box(1), black_box(w), None));
        });

        group.bench_with_input(BenchmarkId::new("plural", name), word, |b, &w| {
            b.iter(|| text::pluralize(black_box(2), black_box(w), None));
        });

        group.bench_with_input(BenchmarkId::new("custom_suffix", name), word, |b, &w| {
            b.iter(|| text::pluralize(black_box(2), black_box(w), Some("ies,y")));
        });
    }

    group.finish();
}

fn ordinal_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("ordinal");

    let numbers = vec![
        1, 2, 3, 4, 11, 12, 13, 21, 22, 23, 100, 101, 102, 111, 112, 113, 1000, 10000, 100000,
        1234567,
    ];

    for num in numbers.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(num), num, |b, &n| {
            b.iter(|| text::ordinal(black_box(n)));
        });
    }

    group.finish();
}

fn intcomma_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("intcomma");

    let numbers = vec![
        ("small", 123),
        ("medium", 123456),
        ("large", 123456789),
        ("very_large", 123456789012),
    ];

    for (name, num) in numbers.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), num, |b, &n| {
            b.iter(|| text::intcomma(black_box(n)));
        });
    }

    group.finish();
}

fn floatcomma_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("floatcomma");

    let numbers = vec![
        ("small", 123.45),
        ("medium", 123456.789),
        ("large", 123456789.123456),
        ("precision", 1.23456789012345),
    ];

    for (name, num) in numbers.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), num, |b, &n| {
            b.iter(|| text::floatcomma(black_box(n), 2));
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    capfirst_benchmarks,
    title_benchmarks,
    pluralize_benchmarks,
    ordinal_benchmarks,
    intcomma_benchmarks,
    floatcomma_benchmarks
);
criterion_main!(benches);

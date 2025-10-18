use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use reinhardt_utils::encoding::*;
use std::hint::black_box;

fn urlencode_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("urlencode");

    let test_cases = vec![
        ("ascii", "hello world"),
        ("mixed", "Hello World! 123 @#$%"),
        ("unicode", "Hello 世界 مرحبا"),
    ];

    for (name, text) in test_cases.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), text, |b, &t| {
            b.iter(|| urlencode(black_box(t)));
        });
    }
    group.finish();
}

fn urldecode_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("urldecode");

    let test_cases = vec![
        ("simple", "hello+world"),
        ("encoded", "Hello%20World%21"),
        ("complex", "test%26value%3D1%2B2"),
    ];

    for (name, text) in test_cases.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), text, |b, &t| {
            b.iter(|| urldecode(black_box(t)));
        });
    }
    group.finish();
}

fn slugify_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("slugify");

    for len in [10, 50, 100].iter() {
        let title = "Hello World 123".repeat(*len);
        group.bench_with_input(BenchmarkId::from_parameter(len), &title, |b, t| {
            b.iter(|| slugify(black_box(t)));
        });
    }
    group.finish();
}

fn truncate_chars_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("truncate_chars");

    let long_text = "This is a very long text that will be truncated ".repeat(100);

    for max_len in [50, 200, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(max_len), max_len, |b, &ml| {
            b.iter(|| truncate_chars(black_box(&long_text), black_box(ml)));
        });
    }
    group.finish();
}

fn wrap_text_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("wrap_text");

    let long_text =
        "This is a very long text that needs to be wrapped at a specific width for proper display "
            .repeat(20);

    for width in [40, 80, 120].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(width), width, |b, &w| {
            b.iter(|| wrap_text(black_box(&long_text), black_box(w)));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    urlencode_benchmarks,
    urldecode_benchmarks,
    slugify_benchmarks,
    truncate_chars_benchmarks,
    wrap_text_benchmarks
);
criterion_main!(benches);

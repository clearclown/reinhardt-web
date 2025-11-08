use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use reinhardt_utils::html::*;
use std::hint::black_box;

fn escape_benchmarks(c: &mut Criterion) {
	let mut group = c.benchmark_group("escape");

	for size in [10, 100, 1000].iter() {
		let html = "<div>Hello &amp; goodbye</div>".repeat(*size);
		group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
			b.iter(|| escape(black_box(&html)));
		});
	}
	group.finish();
}

fn unescape_benchmarks(c: &mut Criterion) {
	let mut group = c.benchmark_group("unescape");

	for size in [10, 100, 1000].iter() {
		let html = "&lt;div&gt;Hello &amp;amp; goodbye&lt;/div&gt;".repeat(*size);
		group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
			b.iter(|| unescape(black_box(&html)));
		});
	}
	group.finish();
}

fn strip_tags_benchmarks(c: &mut Criterion) {
	let mut group = c.benchmark_group("strip_tags");

	for size in [10, 100, 1000].iter() {
		let html = "<p>This is <b>bold</b> and <i>italic</i> text</p>".repeat(*size);
		group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
			b.iter(|| strip_tags(black_box(&html)));
		});
	}
	group.finish();
}

fn truncate_html_words_benchmarks(c: &mut Criterion) {
	let mut group = c.benchmark_group("truncate_html_words");

	let html = "<p>This is a very long piece of HTML content with many words that needs to be truncated for display purposes</p>".repeat(10);

	for words in [10, 50, 100].iter() {
		group.bench_with_input(BenchmarkId::from_parameter(words), words, |b, &w| {
			b.iter(|| truncate_html_words(black_box(&html), black_box(w)));
		});
	}
	group.finish();
}

criterion_group!(
	benches,
	escape_benchmarks,
	unescape_benchmarks,
	strip_tags_benchmarks,
	truncate_html_words_benchmarks
);
criterion_main!(benches);

use chrono::{TimeZone, Utc};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use reinhardt_utils::dateformat;
use std::hint::black_box;

fn format_benchmarks(c: &mut Criterion) {
	let mut group = c.benchmark_group("format");
	let dt = Utc.with_ymd_and_hms(2025, 6, 15, 14, 30, 45).unwrap();

	let formats = vec![
		("simple_date", "Y-m-d"),
		("full_datetime", "Y-m-d H:i:s"),
		("full_text", "l, F j, Y - g:i:s A"),
		("complex", "l, F j, Y \\a\\t g:i:s A (\\U\\T\\C)"),
	];

	for (name, format_str) in formats.iter() {
		group.bench_with_input(BenchmarkId::from_parameter(name), format_str, |b, &fmt| {
			b.iter(|| dateformat::format(black_box(&dt), black_box(fmt)));
		});
	}

	group.finish();
}

fn shortcuts_benchmarks(c: &mut Criterion) {
	let mut group = c.benchmark_group("shortcuts");
	let dt = Utc.with_ymd_and_hms(2025, 6, 15, 14, 30, 45).unwrap();

	group.bench_function("iso_date", |b| {
		b.iter(|| dateformat::shortcuts::iso_date(black_box(&dt)));
	});

	group.bench_function("iso_datetime", |b| {
		b.iter(|| dateformat::shortcuts::iso_datetime(black_box(&dt)));
	});

	group.bench_function("us_date", |b| {
		b.iter(|| dateformat::shortcuts::us_date(black_box(&dt)));
	});

	group.bench_function("eu_date", |b| {
		b.iter(|| dateformat::shortcuts::eu_date(black_box(&dt)));
	});

	group.bench_function("full_date", |b| {
		b.iter(|| dateformat::shortcuts::full_date(black_box(&dt)));
	});

	group.bench_function("short_date", |b| {
		b.iter(|| dateformat::shortcuts::short_date(black_box(&dt)));
	});

	group.bench_function("time_24", |b| {
		b.iter(|| dateformat::shortcuts::time_24(black_box(&dt)));
	});

	group.bench_function("time_12", |b| {
		b.iter(|| dateformat::shortcuts::time_12(black_box(&dt)));
	});

	group.finish();
}

fn escape_benchmarks(c: &mut Criterion) {
	let mut group = c.benchmark_group("escape");
	let dt = Utc.with_ymd_and_hms(2025, 6, 15, 14, 30, 45).unwrap();

	let escape_counts = vec![0, 5, 10, 20];

	for count in escape_counts.iter() {
		let format_str = "Y-".to_string() + &"\\Y".repeat(*count);
		group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, _| {
			b.iter(|| dateformat::format(black_box(&dt), black_box(&format_str)));
		});
	}

	group.finish();
}

fn varied_dates_benchmarks(c: &mut Criterion) {
	let mut group = c.benchmark_group("varied_dates");

	let dates = vec![
		(
			"year_1000",
			Utc.with_ymd_and_hms(1000, 1, 1, 0, 0, 0).unwrap(),
		),
		(
			"year_2000",
			Utc.with_ymd_and_hms(2000, 6, 15, 12, 30, 45).unwrap(),
		),
		(
			"year_2100",
			Utc.with_ymd_and_hms(2100, 12, 31, 23, 59, 59).unwrap(),
		),
		(
			"year_9999",
			Utc.with_ymd_and_hms(9999, 12, 31, 23, 59, 59).unwrap(),
		),
	];

	for (name, dt) in dates.iter() {
		group.bench_with_input(BenchmarkId::from_parameter(name), dt, |b, dt| {
			b.iter(|| dateformat::format(black_box(dt), black_box("l, F j, Y - g:i:s A")));
		});
	}

	group.finish();
}

criterion_group!(
	benches,
	format_benchmarks,
	shortcuts_benchmarks,
	escape_benchmarks,
	varied_dates_benchmarks
);
criterion_main!(benches);

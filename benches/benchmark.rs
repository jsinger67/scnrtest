use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use std::{sync::LazyLock, time::Duration};

use scnr::{Scanner, ScannerBuilder, ScannerMode};

static SCANNER_MODES_DATA: &str = include_str!("../data/veryl_modes.json");

static SCANNER_MODES: LazyLock<Vec<ScannerMode>> =
    LazyLock::new(|| serde_json::from_str(SCANNER_MODES_DATA).unwrap());

static SCANNER: LazyLock<Scanner> = LazyLock::new(|| {
    ScannerBuilder::new()
        .add_scanner_modes(&SCANNER_MODES)
        .build()
        .unwrap()
});

static INPUT: &str = include_str!("../data/input/big2.veryl");

fn build_scanner(c: &mut Criterion) {
    c.bench_function("build_scanner", |b| {
        b.iter(|| {
            let _scanner = black_box(
                ScannerBuilder::new()
                    .add_scanner_modes(&SCANNER_MODES)
                    .build()
                    .unwrap(),
            );
        });
    });
}

fn find_iter(c: &mut Criterion) {
    c.bench_function("find_iter", |b| {
        b.iter(|| {
            let _ = SCANNER.find_iter(INPUT);
        });
    });
}

fn run_scanner(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Bytes(INPUT.len() as u64));
    group.bench_function("scan", |b| {
        b.iter(|| {
            for ma in SCANNER.find_iter(INPUT) {
                black_box(ma);
            }
        });
    });
}

criterion_group! {
    name = build;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = build_scanner, find_iter
}

criterion_group! {
    name = scan;
    config = Criterion::default().sample_size(50);
    targets = run_scanner
}

criterion_main!(build, scan);

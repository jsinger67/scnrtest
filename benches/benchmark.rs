mod v1_veryl_scanner;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use parol_runtime::{ScannerConfig, TokenStream, Tokenizer};
use std::{cell::RefCell, sync::LazyLock, time::Duration};
use v1_veryl_scanner::{MAX_K, SCANNERS, SCANNER_0, SCANNER_1, SCANNER_2, TERMINALS};

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

fn build_scanner_uncached(c: &mut Criterion) {
    c.bench_function("build_scanner_uncached", |b| {
        b.iter(|| {
            let _scanner = black_box(
                ScannerBuilder::new()
                    .add_scanner_modes(&SCANNER_MODES)
                    .build_uncached()
                    .unwrap(),
            );
        });
    });
}

fn build_scanner_v1(c: &mut Criterion) {
    c.bench_function("build_scanner_v1", |b| {
        b.iter(|| {
            let _tokenizers = black_box(vec![
                ScannerConfig::new(
                    "INITIAL",
                    Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
                    &[],
                ),
                ScannerConfig::new(
                    "Embed",
                    Tokenizer::build(TERMINALS, SCANNER_1.0, SCANNER_1.1).unwrap(),
                    &[],
                ),
                ScannerConfig::new(
                    "Generic",
                    Tokenizer::build(TERMINALS, SCANNER_2.0, SCANNER_2.1).unwrap(),
                    &[],
                ),
            ]);
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
                assert_ne!(ma.token_type(), 118, "Invalid token found: {:?}", ma);
                black_box(ma);
            }
        });
    });
}

fn run_scanner_v1(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Bytes(INPUT.len() as u64));
    group.bench_function("scan_v1", |b| {
        b.iter(|| {
            let token_stream = RefCell::new(TokenStream::new(INPUT, "", &SCANNERS, MAX_K).unwrap());
            while !token_stream.borrow().all_input_consumed() {
                let tok = token_stream.borrow_mut().lookahead(0).unwrap();
                assert_ne!(tok.token_type, 118, "Invalid token found: {:?}", tok);
                black_box(tok);
                token_stream.borrow_mut().consume().unwrap();
            }
        });
    });
}

criterion_group! {
    name = build;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = build_scanner, build_scanner_uncached, build_scanner_v1, find_iter
}

criterion_group! {
    name = scan;
    config = Criterion::default().sample_size(50);
    targets = run_scanner, run_scanner_v1
}

criterion_main!(build, scan);

use criterion::{criterion_group, criterion_main, Criterion};
use rustav::{load_yara_rules, scan_file};

fn bench_single_scan(c: &mut Criterion) {
    let rules = load_yara_rules("rules");

    let sample_path = "samples/sample_00001.txt";

    c.bench_function("scan single file", |b| {
        b.iter(|| {
            scan_file(&rules, sample_path);
        })
    });
}

criterion_group!(benches, bench_single_scan);
criterion_main!(benches);

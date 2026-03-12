use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use fasrt::srt::Parser;

const SMALL_SRT: &str = "\
1
00:00:01,000 --> 00:00:04,000
Hello world!

2
00:00:05,000 --> 00:00:08,000
Goodbye world!
";

const MEDIUM_SRT: &str = include_str!("../fixtures/srt/DeathNote_01.eng.srt");

fn load_all_fixtures() -> String {
  if let Ok(read_dir) = std::fs::read_dir("fixtures/srt") {
    let mut paths: Vec<_> = read_dir
      .filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "srt") {
          Some(path)
        } else {
          None
        }
      })
      .collect();

    paths.sort();

    let mut buf = String::new();
    for path in paths {
      if let Ok(content) = std::fs::read_to_string(path) {
        if !buf.is_empty() {
          buf.push_str("\n\n");
        }
        buf.push_str(&content);
      }
    }
    buf
  } else {
    // Fallback: use a small embedded sample when fixtures are unavailable.
    SMALL_SRT.to_string()
  }
}

fn bench_srt_parse(c: &mut Criterion) {
  let all_fixtures = load_all_fixtures();

  let mut group = c.benchmark_group("srt/parse");

  // Small inline SRT
  group.throughput(Throughput::Bytes(SMALL_SRT.len() as u64));
  group.bench_function(BenchmarkId::new("strict", "small_2_cues"), |b| {
    b.iter(|| {
      let count = Parser::strict(black_box(SMALL_SRT)).count();
      black_box(count);
    });
  });

  // Medium real file (~26 KB)
  group.throughput(Throughput::Bytes(MEDIUM_SRT.len() as u64));
  group.bench_function(BenchmarkId::new("strict", "medium_26kb"), |b| {
    b.iter(|| {
      let count = Parser::strict(black_box(MEDIUM_SRT)).count();
      black_box(count);
    });
  });

  // All fixtures (~8 MB)
  group.throughput(Throughput::Bytes(all_fixtures.len() as u64));
  group.bench_function(BenchmarkId::new("lossy", "all_fixtures_8mb"), |b| {
    b.iter(|| {
      let count = Parser::lossy(black_box(&all_fixtures)).count();
      black_box(count);
    });
  });

  group.finish();
}

fn bench_srt_collect(c: &mut Criterion) {
  let mut group = c.benchmark_group("srt/collect");

  // Collect into Vec to measure allocation overhead
  group.throughput(Throughput::Bytes(MEDIUM_SRT.len() as u64));
  group.bench_function("medium_26kb", |b| {
    b.iter(|| {
      let entries: Vec<_> = Parser::strict(black_box(MEDIUM_SRT))
        .collect::<Result<_, _>>()
        .unwrap();
      black_box(entries.len());
    });
  });

  group.finish();
}

criterion_group!(benches, bench_srt_parse, bench_srt_collect);
criterion_main!(benches);

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use fasrt::vtt::Parser;

const SMALL_VTT: &str = "\
WEBVTT

00:00:00.000 --> 00:00:01.000
Hello world!

00:00:01.000 --> 00:00:02.000
Goodbye world!
";

const SETTINGS_VTT: &str = "\
WEBVTT

STYLE
::cue { color: white; }

REGION
id:region1
width:40%
lines:3
regionanchor:0%,100%
viewportanchor:10%,90%
scroll:up

cue-1
00:00:00.000 --> 00:00:01.000 align:start position:10% size:80% line:0 vertical:rl region:region1
<b>Bold</b> and <i>italic</i> text

NOTE This is a comment

00:00:01.000 --> 00:00:05.000
Second cue with <v Roger Bingham>voice tag</v>
";

fn load_all_vtt_fixtures() -> String {
  let mut buf = String::new();
  for dir in &[
    "fixtures/webvtt/wpt-file-parsing",
    "fixtures/webvtt/wpt-cue-parsing",
  ] {
    if let Ok(entries) = std::fs::read_dir(dir) {
      for entry in entries {
        let entry = entry.unwrap();
        if entry.path().extension().is_some_and(|e| e == "vtt") {
          buf.push_str(&std::fs::read_to_string(entry.path()).unwrap());
          buf.push_str("\n\n");
        }
      }
    }
  }
  buf
}

fn bench_vtt_parse(c: &mut Criterion) {
  let all_fixtures = load_all_vtt_fixtures();

  let mut group = c.benchmark_group("vtt/parse");

  // Small inline VTT
  group.throughput(Throughput::Bytes(SMALL_VTT.len() as u64));
  group.bench_function(BenchmarkId::new("parse", "small_2_cues"), |b| {
    b.iter(|| {
      let count = Parser::new(black_box(SMALL_VTT)).count();
      black_box(count);
    });
  });

  // VTT with settings, regions, styles, cue options
  group.throughput(Throughput::Bytes(SETTINGS_VTT.len() as u64));
  group.bench_function(BenchmarkId::new("parse", "with_settings"), |b| {
    b.iter(|| {
      let count = Parser::new(black_box(SETTINGS_VTT)).count();
      black_box(count);
    });
  });

  // All VTT fixtures
  group.throughput(Throughput::Bytes(all_fixtures.len() as u64));
  group.bench_function(BenchmarkId::new("parse", "all_fixtures"), |b| {
    b.iter(|| {
      let count = Parser::new(black_box(&all_fixtures)).count();
      black_box(count);
    });
  });

  group.finish();
}

fn bench_vtt_collect(c: &mut Criterion) {
  let mut group = c.benchmark_group("vtt/collect");

  group.throughput(Throughput::Bytes(SETTINGS_VTT.len() as u64));
  group.bench_function("with_settings", |b| {
    b.iter(|| {
      let blocks: Vec<_> = Parser::new(black_box(SETTINGS_VTT))
        .collect::<Result<_, _>>()
        .unwrap();
      black_box(blocks.len());
    });
  });

  group.finish();
}

criterion_group!(benches, bench_vtt_parse, bench_vtt_collect);
criterion_main!(benches);

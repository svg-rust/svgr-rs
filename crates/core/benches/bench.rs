use std::fs;

use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use svgr_rs::transform;

pub fn criterion_benchmark(c: &mut Criterion) {
  let path = "benches/rspack-logo.svg";
  let rspack_logo = fs::read_to_string(path).unwrap();
  c.bench_function("Rspack logo", |b| {
    b.iter(|| {
      transform(
        rspack_logo.to_string(),
        Default::default(),
        Default::default(),
      )
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize, Throughput};

#[path = "../src/mesh_2d_fn.rs"]
mod mesh_2d_fn;

fn criterion_benchmark(c: &mut Criterion) {
  let rows = 1024usize;
  let cols = 2048usize;
  let size = rows * cols;

  let mut group = c.benchmark_group("matrix_operations");
  // This allows Criterion to report throughput in MB/s
  group.throughput(Throughput::Bytes((size * std::mem::size_of::<f32>()) as u64));

  group.bench_function("transpose_simd", |b| {
    b.iter_batched(
      // Setup: Prepare data outside the timing loop
      || {
        let src: Vec<f32> = (0..size).map(|x| x as f32).collect();
        let dst = vec![0.0; size];
        (src, dst)
      },
      // Routine: Only time the actual transpose
      |(src, mut dst)| {
        mesh_2d_fn::transpose_blocked_simd_f32(black_box(&mut dst), black_box(&src), rows, cols);
      },
      BatchSize::LargeInput, // Use LargeInput for big vectors
    );
  });

  group.bench_function("transpose_blocked", |b| {
    b.iter_batched(
      || {
          let src: Vec<f32> = (0..size).map(|x| x as f32).collect();
          let dst = vec![0.0; size];
          (src, dst)
      },
      |(src, mut dst)| {
          mesh_2d_fn::transpose_blocked_f32(black_box(&mut dst), black_box(&src), rows, cols);
      },
      BatchSize::LargeInput,
    );
  });
  group.finish();
}

// Macro to collect benchmark functions into a group
criterion_group!(benches, criterion_benchmark);
// Macro to generate a main function that runs the groups
criterion_main!(benches);

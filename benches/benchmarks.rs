use calc_creator_lib::create_body;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

struct NullWriter;

impl std::io::Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn create_calculator_body<T: std::io::Write>(w: T, size: u32)
where
    T: std::io::Write + Send + 'static,
{
    // Simulate the body creation logic
    let mut writer = std::io::BufWriter::new(w);
    create_body(&mut writer, size).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create calculator body 100", |b| {
        b.iter(|| create_calculator_body(black_box(NullWriter), 100))
    });

    c.bench_function("create calculator body 500", |b| {
        b.iter(|| create_calculator_body(black_box(NullWriter), 500))
    });

    c.bench_function("create calculator body 1500", |b| {
        b.iter(|| create_calculator_body(black_box(NullWriter), 1500))
    });

    c.bench_function("create calculator body 3000", |b| {
        b.iter(|| create_calculator_body(black_box(NullWriter), 3000))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

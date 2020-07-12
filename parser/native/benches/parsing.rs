use parser::parsing::parser::Parser;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn file_parser(c: &mut Criterion) {
    c.bench_function("File parser", move |b| b.iter(|| Parser::parse("spe.mp4")));
}

criterion_group!(benches, file_parser);
criterion_main!(benches);

// use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion::*;

use iribench::impls::*;
use iribench::test;
use iribench::BenchIRI;

// TODO: add benches for modifying IRI components. right now we just test IRI creation
pub fn criterion_iri_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("iri creation");

    let x = test::POSITIVE_IRIS;
    let mut idx = 0;

    // TODO: instead of iterating over each example, making each take ~15s (3x5s)
    // figure out a way to maybe create batches of a bunch of different IRIs?
    for &(data, meta) in (&x).into_iter().take(5) {
        // If this is not an absolute IRI, we don't have support for that yet.
        if !meta.0 {
            continue;
        }
        group.bench_with_input(BenchmarkId::new("Iri", idx), &idx, |b, _| {
            b.iter(|| <Iri as BenchIRI>::create_from_str(data))
        });
        group.bench_with_input(BenchmarkId::new("IriParsed", idx), &idx, |b, _| {
            b.iter(|| <IriParsed as BenchIRI>::create_from_str(data))
        });
        group.bench_with_input(BenchmarkId::new("IriStr", idx), &idx, |b, _| {
            b.iter(|| <IriStr as BenchIRI>::create_from_str(data))
        });
        idx += 1;
    }

    group.finish();
}

criterion_group!(benches, criterion_iri_bench);
criterion_main!(benches);

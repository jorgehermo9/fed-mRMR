#![allow(unused)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use csv::Reader;
use fed_mrmr::dataset::*;
use std::{error::Error, time::Instant};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("iris", |b| {
        b.iter(|| select_features(black_box("test/assets/iris.data.disc")))
    });
    c.bench_function("test_lung", |b| {
        b.iter(|| select_features(black_box("test/assets/test_lung_s3.csv")))
    });

    c.bench_function("dataset_merge", |b| {
        b.iter(|| merge_datasets(black_box("test/assets/dataset.csv"), 2))
    });
    c.bench_function("test_lung_merge", |b| {
        b.iter(|| merge_datasets(black_box("test/assets/test_lung_s3.csv"), 2))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn merge_datasets(base: &str, partitions: usize) -> Result<(), Box<dyn Error>> {
    let complete = Dataset::new(Reader::from_path(base)?)?;

    let mut partitions_datasets = (1..=partitions)
        .map(|i| Dataset::new(Reader::from_path(format!("{base}.{i}")).unwrap()).unwrap());
    let first_partition = partitions_datasets.next().unwrap();
    let other_partitions = partitions_datasets.collect::<Vec<_>>();
    let merged = first_partition.merge(other_partitions);

    let complete_rank = complete.mrmr_features("class", None);
    let merged_rank = merged.mrmr_features("class", None);
    assert_eq!(merged_rank, complete_rank);
    Ok(())
}
fn select_features(dataset_path: &str) -> Result<(), Box<dyn Error>> {
    let start_matrix = Instant::now();
    let dataset = Dataset::new(Reader::from_path(dataset_path)?)?;
    let _ = dataset.mrmr_features("class", None);

    Ok(())
}

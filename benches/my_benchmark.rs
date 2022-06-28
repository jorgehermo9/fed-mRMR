#![allow(unused)]

use std::{error::Error, time::Instant};
use csv::Reader;
use mrmr_enhanced::dataset::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("iris", |b| b.iter(|| select_features(black_box("test/assets/iris.data.disc"))));
	c.bench_function("test_lung", |b| b.iter(|| select_features(black_box("test/assets/test_lung_s3.csv"))));
	c.bench_function("synthetic", |b| b.iter(|| select_features(black_box("test/assets/dataset.csv"))));
	c.bench_function("merge_synthetic", |b| b.iter(|| merge_datasets_synthetic()));
	c.bench_function("merge_lung", |b| b.iter(|| merge_datasets_lung()));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


fn merge_datasets_synthetic()-> Result<(), Box<dyn Error>>{
	let complete = Dataset::new(Reader::from_path("test/assets/dataset.csv")?)?;
	let partial_1 = Dataset::new(Reader::from_path("test/assets/dataset-1.csv")?)?;
	let partial_2 = Dataset::new(Reader::from_path("test/assets/dataset-2.csv")?)?;

	let merged = partial_1.merge(partial_2);
	let complete_rank = complete.mrmr_features("class", None);
	let merged_rank = merged.mrmr_features("class", None);
	Ok(())
}
fn merge_datasets_lung()-> Result<(), Box<dyn Error>>{
	let complete = Dataset::new(Reader::from_path("test/assets/test_lung_s3.csv")?)?;
	let partial_1 = Dataset::new(Reader::from_path("test/assets/test_lung_s3-1.csv")?)?;
	let partial_2 = Dataset::new(Reader::from_path("test/assets/test_lung_s3-2.csv")?)?;

	let merged = partial_1.merge(partial_2);
	let complete_rank = complete.mrmr_features("class", None);
	let merged_rank = merged.mrmr_features("class", None);
	Ok(())
}
fn select_features(dataset_path: &str)-> Result<(),Box<dyn Error>>{

	let start_matrix = Instant::now();
	let dataset = Dataset::new(Reader::from_path(dataset_path)?)?;

	let duration_matrix = start_matrix.elapsed();
	let start_mrmr = Instant::now();
	let _ = dataset.mrmr_features("class",None);
	let duration_mrmr = start_mrmr.elapsed();
	// println!("\nElapsed time for matrix construction: {}s",duration_matrix.as_secs_f32());
	// println!("Elapsed time for mrmr calculation: {}s",duration_mrmr.as_secs_f32());
	// println!("Total elapsed time: {}s",(duration_mrmr+duration_matrix).as_secs_f32());
	
	Ok(())
}
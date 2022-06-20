#![allow(unused)]

use std::{error::Error, time::Instant};
use csv::Reader;
use mrmr_enhanced::dataset::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("iris", |b| b.iter(|| select_features(black_box("datasets/iris.data.disc"))));
	c.bench_function("test_lung", |b| b.iter(|| select_features(black_box("datasets/test_lung_s3.csv"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);



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
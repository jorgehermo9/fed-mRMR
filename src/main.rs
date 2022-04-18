extern crate nalgebra as na;
use std::time::{Instant};
use std::io;
use std::process;
use csv::Reader;

mod dataset;
use dataset::Dataset;

fn main() {
	let start_matrix = Instant::now();
	let dataset = match Dataset::new(Reader::from_reader(io::stdin())){
		Err(err) =>{
			println!("error running example: {}", err);
			process::exit(1);
		},
		Ok(dataset) => dataset
	};
	let duration_matrix = start_matrix.elapsed();
	// for header in &dataset.headers{
	// 	println!("{header: }");
	// 	for subheader in &dataset.sub_headers[header]{
	// 		println!("\t{subheader}:{}",dataset.intersection(subheader,subheader).unwrap())
	// 	}
	// }

	let start_mrmr = Instant::now();

	let selected_features = dataset.mrmr_features();
	for (index,(feature,value)) in selected_features.into_iter().enumerate(){
		if feature == "class"{continue}
		println!("{}. {} -> {}",index+1,feature,value);
	}
	let duration_mrmr = start_mrmr.elapsed();
	println!("Elapsed time for matrix construction: {}s",duration_matrix.as_secs_f32());
	println!("Elapsed time for mrmr calculation: {}s",duration_mrmr.as_secs_f32());
	println!("Total elapsed time: {}s",(duration_mrmr+duration_matrix).as_secs_f32())


}

#[cfg(test)]
mod tests{
	use super::*;
	use std::error::Error;

	fn calc_mrmr_dataset(dataset_path: &str)-> Result<(),Box<dyn Error>>{

		let start_matrix = Instant::now();
		let dataset = Dataset::new(Reader::from_path(dataset_path)?)?;

		let duration_matrix = start_matrix.elapsed();
		let start_mrmr = Instant::now();
		let _ = dataset.mrmr_features();
		let duration_mrmr = start_mrmr.elapsed();
		println!("\nElapsed time for matrix construction: {}s",duration_matrix.as_secs_f32());
		println!("Elapsed time for mrmr calculation: {}s",duration_mrmr.as_secs_f32());
		println!("Total elapsed time: {}s",(duration_mrmr+duration_matrix).as_secs_f32());
		
		Ok(())
	}

	#[test]
	fn test_iris()-> Result<(), Box<dyn Error>>{
		calc_mrmr_dataset("test/datasets/iris.data.disc")?;
		Ok(())
	}

	#[test]
	fn test_connect_4()-> Result<(), Box<dyn Error>>{
		calc_mrmr_dataset("test/datasets/connect-4.data")?;
		Ok(())
	}

	#[test]
	fn test_lung()-> Result<(), Box<dyn Error>>{
		calc_mrmr_dataset("test/datasets/test_lung_s3.csv")?;
		Ok(())
	}
}
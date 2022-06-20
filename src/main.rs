use std::error::Error;
use std::time::Instant;
use std::io;
use csv::Reader;

mod dataset;
use dataset::Dataset;

fn main() -> Result<(), Box<dyn Error>>{

	let start_matrix = Instant::now();
	let dataset = Dataset::new(Reader::from_reader(io::stdin()))?;
	let duration_matrix = start_matrix.elapsed();
	
	// println!("Calculated dataset");
	// println!("{:?}",dataset.get_headers().iter().flat_map(|header| dataset.get_header_values(header).unwrap()).collect::<Vec<_>>());
	// println!("{}",dataset.get_matrix());

	// Save to disk
	// dataset.save(&PathBuf::from("dataset.serde"))?;
	
	// Load from disk
	// let start_from_disk = Instant::now();
	// let dataset_disk = Dataset::from(&PathBuf::from("dataset.serde"))?;
	// let duration_from_disk = start_from_disk.elapsed();
	// println!("{:?}",dataset_disk.get_headers().iter().flat_map(|header| dataset_disk.get_header_values(header).unwrap()).collect::<Vec<_>>());
	// println!("From disk dataset");
	// println!("{:?}",dataset_disk.get_matrix());


	let start_merge = Instant::now();
	// let dataset = dataset.merge(dataset_disk);
	let duration_merge = start_merge.elapsed();
	// println!("Merged dataset");
	// println!("{:?}",dataset.get_headers().iter().flat_map(|header| dataset.get_header_values(header).unwrap()).collect::<Vec<_>>());
	// println!("{:?}",dataset.get_matrix());

	

	let start_mrmr = Instant::now();

	let selected_features = dataset.mrmr_features("class",None);
	for (index,(feature,value)) in selected_features.into_iter().enumerate(){
		if feature == "class"{continue}
		println!("{}. {} -> {}",index+1,feature,value);
	}
	let duration_mrmr = start_mrmr.elapsed();


	println!("Elapsed time for matrix construction: {}s",duration_matrix.as_secs_f32());
	// println!("Elapsed time for matrix loading from disk: {}s",duration_from_disk.as_secs_f32());
	println!("Elapsed time for matrix merge: {}s",duration_merge.as_secs_f32());

	println!("Elapsed time for mrmr calculation: {}s",duration_mrmr.as_secs_f32());
	println!("Total elapsed time: {}s",(duration_mrmr+duration_matrix+duration_merge).as_secs_f32());

	Ok(())
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
		let _ = dataset.mrmr_features("class",None);
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

	// #[test]
	// fn test_connect_4()-> Result<(), Box<dyn Error>>{
	// 	calc_mrmr_dataset("test/datasets/connect-4.data")?;
	// 	Ok(())
	// }

	#[test]
	fn test_lung()-> Result<(), Box<dyn Error>>{
		calc_mrmr_dataset("test/datasets/test_lung_s3.csv")?;
		Ok(())
	}
}
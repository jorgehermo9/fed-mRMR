use std::fs;
use std::{error::Error, path::PathBuf};
use std::time::Instant;
use std::io::{self, BufReader, BufRead};
use csv::Reader;
use clap::{Parser,Subcommand};

use mrmr_enhanced::dataset::*;

#[derive(Parser)]
#[clap(author,version,about,long_about = None)]
struct Cli{
	
	#[clap(subcommand)]
	command: Commands
}
#[derive(Subcommand)]
enum Commands{
	Mrmr{
		/// read file from provided path. If none is provided, file is read from stdin
		path:Option<PathBuf>,
		/// flag to try to parse input file as csv format
		#[clap(short,long,action)]
		csv:bool,
	}
}

fn mrmr(path:&Option<PathBuf>,csv:bool)-> Result<(),Box<dyn Error>>{
	let mut reader:Box<dyn BufRead> = match path{
		Some(path) =>Box::new(BufReader::new(fs::File::open(path)?)),
		None=>Box::new(BufReader::new(io::stdin()))
	};
	let dataset = if csv{
		//If csv flag specified and no path is provided, read csv from stdin
		let start_matrix = Instant::now();
		let dataset = Dataset::new(Reader::from_reader(reader))?;
		let duration_matrix = start_matrix.elapsed();
		println!("Elapsed time for matrix construction: {}s",duration_matrix.as_secs_f32());		

		dataset
	}else{
		Dataset::from_reader(&mut reader)?
	};

	let start_mrmr = Instant::now();
	let selected_features = dataset.mrmr_features("class",None);
	for (index,(feature,value)) in selected_features.into_iter().enumerate(){
		if feature == "class"{continue}
		println!("{}. {} -> {}",index+1,feature,value);
	}
	let duration_mrmr = start_mrmr.elapsed();
	println!("Elapsed time for mrmr calculation: {}s",duration_mrmr.as_secs_f32());

	Ok(())
}
fn main() -> Result<(), Box<dyn Error>>{
	let cli = Cli::parse();
	match &cli.command{
		Commands::Mrmr{path,csv}=>mrmr(path,*csv)?
	};
	
	// println!("Calculated dataset");
	// println!("{:?}",dataset.get_headers().iter().flat_map(|header| dataset.get_header_values(header).unwrap()).collect::<Vec<_>>());
	// println!("{}",dataset.get_matrix());

	// Save to disk
	// dataset.save("dataset.mrmr")?;
	
	// Load from disk
	// let start_from_disk = Instant::now();
	// let dataset_disk = Dataset::try_from("dataset.mrmr")?;
	// let duration_from_disk = start_from_disk.elapsed();
	// println!("{:?}",dataset_disk.get_headers().iter().flat_map(|header| dataset_disk.get_header_values(header).unwrap()).collect::<Vec<_>>());
	// println!("From disk dataset");
	// println!("{:?}",dataset_disk.get_matrix());


	// let start_merge = Instant::now();
	// let dataset = dataset.merge(dataset_disk);
	// let duration_merge = start_merge.elapsed();
	// // println!("Merged dataset");
	// // println!("{:?}",dataset.get_headers().iter().flat_map(|header| dataset.get_header_values(header).unwrap()).collect::<Vec<_>>());
	// // println!("{:?}",dataset.get_matrix());

	

	// let start_mrmr = Instant::now();

	// let selected_features = dataset.mrmr_features("class",None);
	// for (index,(feature,value)) in selected_features.into_iter().enumerate(){
	// 	if feature == "class"{continue}
	// 	println!("{}. {} -> {}",index+1,feature,value);
	// }
	// let duration_mrmr = start_mrmr.elapsed();


	// println!("Elapsed time for matrix construction: {}s",duration_matrix.as_secs_f32());
	// // println!("Elapsed time for matrix loading from disk: {}s",duration_from_disk.as_secs_f32());
	// println!("Elapsed time for matrix merge: {}s",duration_merge.as_secs_f32());

	// println!("Elapsed time for mrmr calculation: {}s",duration_mrmr.as_secs_f32());
	// println!("Total elapsed time: {}s",(duration_mrmr+duration_matrix+duration_merge).as_secs_f32());

	Ok(())
}

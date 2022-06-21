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
		#[clap(long)]
		csv:bool,
		//specifiy class feature name
		#[clap(short,long)]
		class:String,
		/// max number of features to select
		#[clap(short,long)]
		num_features:Option<usize>,
	},
	Matrix{
		/// read file from provided path. If none is provided, file is read from stdin
		path:Option<PathBuf>,
		/// path to write matrix
		#[clap(short,long)]
		output:PathBuf,
	},
	Merge{
		/// datasets to merge
		#[clap(required=true,min_values=2)]
		datasets:Vec<PathBuf>,

		/// path to write matrix
		#[clap(short,long)]
		output:PathBuf,
	},
	Show{
		/// read file from provided path. If none is provided, file is read from stdin
		path:Option<PathBuf>,
	}
}

fn mrmr(path:&Option<PathBuf>,csv:bool,class:&String,limit:&Option<usize>)-> Result<(),Box<dyn Error>>{
	let reader:Box<dyn BufRead> = match path{
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
		Dataset::from_reader(reader)?
	};

	let start_mrmr = Instant::now();
	let selected_features = dataset.mrmr_features(class,limit.clone());

	let feature_padding = dataset.get_headers().iter().map(|s|s.len()).max().unwrap();
	let rank_padding = (selected_features.len() as f32).log10() as usize +2;
	for (index,(feature,value)) in selected_features.into_iter().enumerate(){
		if feature == *class{continue}
		let rank = format!("{}.",index+1);
		println!("{rank:<rank_padding$} {feature:<feature_padding$} -> {value:.6}");
	}
	let duration_mrmr = start_mrmr.elapsed();
	println!("Elapsed time for mrmr calculation: {}s",duration_mrmr.as_secs_f32());
	
	Ok(())
}

fn matrix(path:&Option<PathBuf>,output:&PathBuf) -> Result<(),Box<dyn Error>>{
	let reader:Box<dyn BufRead> = match path{
		Some(path) =>Box::new(BufReader::new(fs::File::open(path)?)),
		None=>Box::new(BufReader::new(io::stdin()))
	};
	let dataset = Dataset::new(Reader::from_reader(reader))?;
	dataset.save(output)?;
	println!("Matrix saved to {}",output.display());

	Ok(())
}

fn merge(datasets:&Vec<PathBuf>,output:&PathBuf) -> Result<(),Box<dyn Error>> {

	let mut paths = datasets.iter();
	let first_path = paths.next().unwrap();
	let mut result = Dataset::from_path(first_path)?;
	for path in paths{
		let dataset =  Dataset::from_path(path)?;
		result = result.merge(dataset);
	}
	result.save(output)?;
	println!("Merged dataset matrix saved to {}",output.display());

	Ok(())
}

fn show(path:&Option<PathBuf>) -> Result<(),Box<dyn Error>> {
	let reader:Box<dyn BufRead> = match path{
		Some(path) =>Box::new(BufReader::new(fs::File::open(path)?)),
		None=>Box::new(BufReader::new(io::stdin()))
	};
	let dataset =  Dataset::from_reader(reader)?;

	let features = dataset.get_headers();
	let sub_features = dataset.get_subheaders();
	let mut num_subfeatures =0;
	for feature in features{
		for sub_feature in sub_features.get(feature).unwrap(){
			print!("{sub_feature} ");
			num_subfeatures+=1;
		}
	}
	println!();
	println!("\n{num_subfeatures} sub_features");
	println!("{} instances",dataset.get_instances());
	let matrix = dataset.get_matrix();
	println!("{matrix}");
	Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
	let cli = Cli::parse();
	match &cli.command{
		Commands::Mrmr{path,csv,class,num_features}=>mrmr(path,*csv,class,num_features)?,
		Commands::Matrix{path,output} => matrix(path,output)?,
		Commands::Merge{datasets,output} => merge(datasets,output)?,
		Commands::Show{path} => show(path)?,
	};

	Ok(())
}

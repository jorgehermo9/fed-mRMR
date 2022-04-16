extern crate nalgebra as na;

use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::process;
use na::{Dynamic, OMatrix};

type IMatrix = OMatrix<i32, Dynamic, Dynamic>;

fn example() -> Result<(), Box<dyn Error>> {
	let mut rdr = csv::Reader::from_reader(io::stdin());
	let headers = rdr.headers()?.to_owned();
	let instances:Vec<_> = (rdr.records()).into_iter()
		.filter_map(|record| record.ok())
		.collect();

	let mut new_headers = vec![];
	let mut onehot = vec![];
	for (index,header) in headers.iter().enumerate(){
		
		let unique_values = instances.iter()
			.map(|record| record.get(index).unwrap())
			.collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();

		
		let mut current_onehot:Vec<_> = unique_values.iter()
			.map(|&value | 
				instances.iter().map(move |i| if i.get(index).unwrap() == value {1}else{0}).collect::<Vec<_>>())
			.flatten().collect();
		
		onehot.append(&mut current_onehot);

		for subheader in unique_values.iter(){
			new_headers.push(format!("{header}_{subheader}"))
		}
	}
	let matrix = IMatrix::from_vec(instances.len(),new_headers.len(),onehot);
	let transpose = matrix.transpose();

	//Intersection of features is the product of A' * A
	let result = transpose * (&matrix);
	println!("{result}");

	Ok(())
}

fn main() {
	if let Err(err) = example() {
		println!("error running example: {}", err);
		process::exit(1);
	}
}
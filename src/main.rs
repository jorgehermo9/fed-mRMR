extern crate nalgebra as na;

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::process;
use na::{Dynamic, OMatrix};

type IMatrix = OMatrix<i32, Dynamic, Dynamic>;

//TODO: change sub_headers to HashMap<String,Vec<String>>
struct Dataset{
	headers:Vec<String>,
	sub_headers:Vec<String>,
	positions:HashMap<String,usize>,
	instances:usize,
	matrix:IMatrix,
}

impl Dataset{
	pub fn get(&self,feature_a:&str,feature_b:&str)->Option<i32>{
		let cell = match (self.positions.get(feature_a),self.positions.get(feature_b)){
			(Some(index_a),Some(index_b))=>(*index_a,*index_b),
			_=>return None,
		};

		return self.matrix.get(cell).cloned();
	}
}
fn intersection_matrix() -> Result<Dataset,Box<dyn Error>>{
	let mut rdr = csv::Reader::from_reader(io::stdin());
	let headers = rdr.headers()?.iter()
	.map(|header| header.to_string())
	.collect::<Vec<_>>();

	let instances:Vec<_> = (rdr.records()).into_iter()
		.filter_map(|record| record.ok())
		.collect();

	let mut sub_headers = vec![];
	let mut onehot = vec![];
	for (index,header) in headers.iter().enumerate(){
		
		let unique_values = instances.iter()
			.map(|record| record.get(index).unwrap())
			.collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();

		
		let mut current_onehot:Vec<_> = unique_values.iter()
			.flat_map(|&value | 
				instances.iter().map(move |i| if i.get(index).unwrap() == value {1}else{0}))
			.collect();
		
		onehot.append(&mut current_onehot);

		for subheader in unique_values.iter(){
			sub_headers.push(format!("{header}_{subheader}"))
		}
	}
	let matrix = IMatrix::from_vec(instances.len(),sub_headers.len(),onehot);
	let transpose = matrix.transpose();

	//Intersection of features is the product of A' * A
	let result = transpose * (&matrix);

	let positions = sub_headers.iter().enumerate()
	.map(|(index,value)| (value.to_string(),index))
	.collect::<HashMap<_,_>>();


	return Ok(Dataset{
		headers,
		sub_headers,
		positions,
		instances:instances.len(),
		matrix:result
	})
}

fn calculate(dataset:Dataset){

	println!("{:?}",dataset.get("fuma_no","transporte_autobus"));

}

fn main() {
	match intersection_matrix(){
		Err(err) =>{
			println!("error running example: {}", err);
			process::exit(1);
		},
		Ok(dataset) => calculate(dataset)
	}
}
extern crate nalgebra as na;

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::error::Error;

use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;
use csv::Reader;

use na::DMatrix;
use nalgebra_sparse::csc::CscMatrix;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dataset{
	headers:Vec<String>,
	subheaders:HashMap<String,Vec<String>>,
	positions:HashMap<String,usize>,
	instances:usize,
	matrix:DMatrix<isize>
}

#[derive(Debug)]
struct MrmrInfo{
	relevance:f64,
	redundance:f64,
	mrmr:f64,
	feature:String
}
// Function to write onehotmatrix to binary file in graph format
// fn write_one_hot(matrix: &IMatrix,subheaders:&Vec<String>){

// 	let mut bin_file = File::create("bitmap.bitmap").unwrap();
// 	let mut headers_file = File::create("headers.headers").unwrap();

// 	for subheader in subheaders{
// 		headers_file.write_all(format!("{} ",subheader).as_bytes()).unwrap();
// 	}
// 	let nodes = matrix.ncols();
// 	let instances = matrix.nrows();
// 	// print!("{} ", nodes as i32);
// 	// print!("{} ",matrix.sum() as i64);

// 	bin_file.write_all(&(nodes as i32).to_le_bytes()).unwrap();
// 	bin_file.write_all(&(matrix.sum() as i64).to_le_bytes()).unwrap();


// 	for i in 0..nodes{
// 		// print!("{} ",((i+1) as i32) *-1);
// 		bin_file.write_all(&(((i+1) as i32)*-1).to_le_bytes()).unwrap();

// 		for j in 0..instances{
// 			if *matrix.get((j,i)).unwrap() == 1{
// 				// print!("{} ",(j+1) as i32);
// 				bin_file.write_all(&((j+1) as i32).to_le_bytes()).unwrap();
// 			}
// 		}
// 	}
// }
impl Dataset{

	pub fn new <R: io::Read> (reader: Reader<R>)-> Result<Self,Box<dyn Error>>{
		let mut rdr = reader;
		let headers = rdr.headers()?.iter()
		.map(|header| header.to_string())
		.collect::<Vec<_>>();

		let instances:Vec<_> = (rdr.records()).into_iter()
			.filter_map(|record| record.ok())
			.collect();

		let mut sub_headers = vec![];
		let mut sub_headers_map = HashMap::new();
		let mut data = vec![];
		let mut col_offsets=vec![0];
		let mut row_indexes = vec![];

		for (index,header) in headers.iter().enumerate(){
			let unique_values = instances.iter()
				.map(|record| record.get(index).unwrap())
				.collect::<BTreeSet<_>>();
				// .into_iter().collect::<Vec<_>>();

			for value in unique_values.iter(){
				for (j,instance) in instances.iter().enumerate(){
					if instance.get(index).unwrap() == *value{
						data.push(1);
						row_indexes.push(j);
					}
				}
				col_offsets.push(row_indexes.len())
			}
			

			sub_headers.extend(unique_values.iter().map(|subheader|format!("{header}_{subheader}")));
			sub_headers_map.insert(header.to_string(),unique_values.iter().map(|subheader|format!("{header}_{subheader}")).collect());
			
		}
		let sparse_matrix = CscMatrix::try_from_csc_data(
			instances.len(), sub_headers.len(), col_offsets, row_indexes, data)
			.expect("Could not create sparse matrix: Invalid csc data");

		let result = sparse_matrix.transpose() * sparse_matrix;
		let result = DMatrix::from(&result);

		let positions = sub_headers.iter().enumerate()
			.map(|(index,value)| (value.to_string(),index))
			.collect::<HashMap<_,_>>();

		// write_one_hot(&matrix,&sub_headers);
		return Ok(Dataset{
			headers,
			subheaders:sub_headers_map,
			positions,
			instances:instances.len(),
			matrix:result,
		})
	}
	pub fn intersection(&self, sub_feature_a: &str, sub_feature_b: &str)->Option<isize>{
		let cell = match (self.positions.get(sub_feature_a),self.positions.get(sub_feature_b)){
			(Some(index_a),Some(index_b))=>(*index_a,*index_b),
			_=>return None,
		};
		return self.matrix.get(cell).cloned();
	}

	pub fn mutual_info(&self,feature_a: &str, feature_b: &str) -> Option<f64>{

		let sub_headers = self.get_subheaders();
		let (sub_features_a,sub_features_b) =match (sub_headers.get(feature_a), sub_headers.get(feature_b)){
			(Some(sub_features_a),Some(sub_features_b)) => (sub_features_a,sub_features_b),
			_=> return None
		};

		let pairs = sub_features_a.iter()
			.flat_map(|sub_feature_a| sub_features_b.iter().map(move |sub_feature_b| (sub_feature_a,sub_feature_b)));

		let mut m_info=0.0;
		for (feature_a,feature_b) in pairs{
			let a_prob = self.intersection(feature_a,feature_a).unwrap() as f64 /self.instances as f64;
			let b_prob = self.intersection(feature_b,feature_b).unwrap()as f64 /self.instances as f64;
			let a_and_b_prob = self.intersection(feature_a,feature_b).unwrap() as f64/self.instances as f64;
			
			//Cannot compute log2(0)
			if a_and_b_prob == 0.0{continue};

			m_info += a_and_b_prob * (a_and_b_prob/(a_prob*b_prob)).log2();
		}
		return Some(m_info);
	}

	pub fn mrmr_features(&self,class:&str,limit:Option<usize>)->Vec<(String,f64)>{
		//Dont return most relevant if num_features is 0
		if limit == Some(0){
			return vec![]
		};
		
		let features = self.get_headers().iter().filter(|f| *f!=class);
		
		let mut mrmr_info_vec = Vec::new();
		
		let mut max_mi = f64::MIN;
		let mut max_index = 0;
		for (index,feature) in features.enumerate() {
			let mi = self.mutual_info(feature,class).unwrap();
			if mi > max_mi{
				max_mi = mi;
				max_index =index; 
			}
			mrmr_info_vec.push(MrmrInfo{
				relevance:mi,
				redundance:0.0_f64,
				mrmr:mi,
				feature:feature.to_string()
			});
		}

		let most_relevant = mrmr_info_vec.swap_remove(max_index);
		let mut selected_features = vec![most_relevant];

		let max_num_features = self.get_headers().len()-1;
		let num_features = match limit{
			Some(n) =>if n > max_num_features {max_num_features} else {n},
			None => max_num_features
		};
		for _ in 0..num_features-1{
			
			let mut max_value = f64::MIN;
			let mut max_index = 0;
			for (index,feature_info) in mrmr_info_vec.iter_mut().enumerate() {
				//update redundance of feature compared to the last selected feature
				feature_info.redundance += self.mutual_info(&feature_info.feature, &selected_features.last().unwrap().feature).unwrap();

				let mrmr_value =  feature_info.relevance - feature_info.redundance/(selected_features.len() as f64);
				feature_info.mrmr = mrmr_value;
				//keep track of feature with maximum mrmr value
				if mrmr_value > max_value{
					max_value = mrmr_value;
					max_index = index;
				}
			}
			let most_mrmr = mrmr_info_vec.swap_remove(max_index);
			selected_features.push(most_mrmr);
		}

		return selected_features.into_iter().map(|feature_info| (feature_info.feature,feature_info.mrmr)).collect::<Vec<_>>();
	}

	pub fn save<P: Sized +AsRef<Path>> (&self,path: P) ->Result<(), Box<dyn Error>>{
		let content = serde_json::to_string(self).unwrap();
		let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&path)?;
		Ok(file.write_all(&content.as_bytes())?)
	}

	pub fn from_path<P: Sized +AsRef<Path>> (path: P) -> Result<Self,Box<dyn Error>> {
		let file =  File::open(&path)?;
		return Self::from_reader(&mut io::BufReader::new(file))
	}
	pub fn from_reader<T:io::Read>(mut reader: T) -> Result<Self,Box<dyn Error>>{
		let mut buff = String::new();
		reader.read_to_string(&mut buff)?;
		return Ok(serde_json::from_str(&buff)?);
	}
	

	pub fn get_headers(&self) -> &Vec<String>{
		&self.headers
	}
	pub fn get_instances(&self) -> usize{
		self.instances
	}
	pub fn get_matrix(&self) ->&DMatrix<isize>{
		&self.matrix
	}
	pub fn get_subheaders(&self) ->&HashMap<String,Vec<String>>{
		&self.subheaders
	}
	pub fn get_header_values(&self,header:&str)->Option<&Vec<String>>{
		self.get_subheaders().get(header)
	}
}

pub trait Merge<T>{
	fn merge(self,other:T) -> Self;
}
impl Merge<Dataset> for Dataset{
	fn merge(self,other:Dataset) ->Self{
		let instances = self.get_instances() + other.get_instances();

		// BTreeSet does not preserve insertion order; it orders the string alphabetically
		let headers = self.get_headers().clone().into_iter()
			.chain(other.get_headers().clone().into_iter()).collect::<BTreeSet<_>>()
			.into_iter().collect::<Vec<_>>();


		let mut subheaders_map = HashMap::new();
		let self_subheaders = self.get_subheaders();
		let to_merge_subheaders = other.get_subheaders();

		for header in headers.iter(){
			let vec = {
				let default = vec![];
				let a = self_subheaders.get(header).unwrap_or(&default);
				let b =to_merge_subheaders.get(header).unwrap_or(&default);
				
				a.clone().into_iter().chain(b.clone().into_iter())
					.collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>()
				
			};
			subheaders_map.insert(header.to_string(),vec);
		}
		let flat_subheaders  =headers.iter()
			.flat_map(|header| subheaders_map.get(header).unwrap())
			.collect::<Vec<_>>();


		let positions = flat_subheaders.iter().enumerate()
			.map(|(index,value)| (value.to_string(),index))
			.collect::<HashMap<_,_>>();
		

		let subheaders_iter = flat_subheaders.iter().
			flat_map(|subheader_a| flat_subheaders.iter().map(move |subheader_b| (subheader_a,subheader_b)))
			.map(|(subheader_a,subheader_b)|{
				self.intersection(subheader_a, subheader_b).unwrap_or(0)
					+ other.intersection(subheader_a, subheader_b).unwrap_or(0)
			});
		let num_subheaders = flat_subheaders.len();
		let matrix = DMatrix::from_iterator(num_subheaders, num_subheaders,subheaders_iter);
		// let matrix = CscMatrix::from(&matrix);

		Dataset{
			headers,
			subheaders:subheaders_map,
			positions,
			instances,
			matrix
		}
	}
}
impl Merge<Vec<Dataset>> for Dataset{
	fn merge(self,other:Vec<Dataset>) -> Dataset{
		let mut result = self;
		for to_merge in other{
			result = result.merge(to_merge)
		};
		return result;
	}
}


#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn merge_datasets_synthetic()-> Result<(), Box<dyn Error>>{
		let complete = Dataset::new(Reader::from_path("test/assets/dataset.csv")?)?;
		let partial_1 = Dataset::new(Reader::from_path("test/assets/dataset-1.csv")?)?;
		let partial_2 = Dataset::new(Reader::from_path("test/assets/dataset-2.csv")?)?;

		let merged = partial_1.merge(partial_2);
		let complete_rank = complete.mrmr_features("class", None);
		let merged_rank = merged.mrmr_features("class", None);

		assert_eq!(merged_rank,complete_rank);

		Ok(())
	}

	#[test]
	fn merge_datasets_microarray()-> Result<(), Box<dyn Error>>{
		let complete = Dataset::new(Reader::from_path("test/assets/test_lung_s3.csv")?)?;
		let partial_1 = Dataset::new(Reader::from_path("test/assets/test_lung_s3-1.csv")?)?;
		let partial_2 = Dataset::new(Reader::from_path("test/assets/test_lung_s3-2.csv")?)?;

		let merged = partial_1.merge(partial_2);
		let complete_rank = complete.mrmr_features("class", None);
		let merged_rank = merged.mrmr_features("class", None);

		assert_eq!(merged_rank,complete_rank);

		Ok(())
	}
	// use std::{error::Error, time::Instant};

	// fn calc_mrmr_dataset(dataset_path: &str)-> Result<(),Box<dyn Error>>{

	// 	let start_matrix = Instant::now();
	// 	let dataset = Dataset::new(Reader::from_path(dataset_path)?)?;

	// 	let duration_matrix = start_matrix.elapsed();
	// 	let start_mrmr = Instant::now();
	// 	let _ = dataset.mrmr_features("class",None);
	// 	let duration_mrmr = start_mrmr.elapsed();
	// 	println!("\nElapsed time for matrix construction: {}s",duration_matrix.as_secs_f32());
	// 	println!("Elapsed time for mrmr calculation: {}s",duration_mrmr.as_secs_f32());
	// 	println!("Total elapsed time: {}s",(duration_mrmr+duration_matrix).as_secs_f32());
		
	// 	Ok(())
	// }

	// #[test]
	// fn test_iris()-> Result<(), Box<dyn Error>>{
	// 	calc_mrmr_dataset("test/datasets/iris.data.disc")?;
	// 	Ok(())
	// }

	// // #[test]
	// // fn test_connect_4()-> Result<(), Box<dyn Error>>{
	// // 	calc_mrmr_dataset("test/datasets/connect-4.data")?;
	// // 	Ok(())
	// // }

	// #[test]
	// fn test_lung()-> Result<(), Box<dyn Error>>{
	// 	calc_mrmr_dataset("test/datasets/test_lung_s3.csv")?;
	// 	Ok(())
	// }
}
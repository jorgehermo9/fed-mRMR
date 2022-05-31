extern crate nalgebra as na;

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::error::Error;
// use std::fs::File;
// use std::io::Write;

use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use csv::Reader;

use na::{Dynamic, OMatrix};
use nalgebra_sparse::csc::CscMatrix;

use serde::{Serialize, Deserialize};



type IMatrix = OMatrix<isize, Dynamic, Dynamic>;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dataset{
	headers:Vec<String>,
	subheaders:HashMap<String,Vec<String>>,
	positions:HashMap<String,usize>,
	instances:usize,
	matrix:IMatrix
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
		let mut onehot = vec![];
		for (index,header) in headers.iter().enumerate(){
			let unique_values = instances.iter()
				.map(|record| record.get(index).unwrap())
				.collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>();

			
			let mut current_onehot:Vec<_> = unique_values.iter()
				.flat_map(|&value | 
					instances.iter().map(move |i| if i.get(index).unwrap() == value {1}else{0}))
				.collect();
			
			onehot.append(&mut current_onehot);

			sub_headers.extend(unique_values.iter().map(|subheader|format!("{header}_{subheader}")));

			sub_headers_map.insert(header.to_string(),unique_values.iter().map(|subheader|format!("{header}_{subheader}")).collect());
			
		}

		
		
		
		let matrix = IMatrix::from_vec(instances.len(),sub_headers.len(),onehot);
		let sparse_matrix = CscMatrix::from(&matrix);
		println!("sparse -> total: {}, non-zero: {}",sparse_matrix.nrows()*sparse_matrix.ncols(),sparse_matrix.nnz());
		//Intersection of features is the product of A' * A		
		// let result = matrix.tr_mul(&matrix);
		let result = sparse_matrix.transpose() * sparse_matrix;
		println!("result -> total: {}, non-zero: {}",result.nrows()*result.ncols(),result.nnz());
		let result = IMatrix::from(&result);

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

		// return Some(self.matrix.column(a).dot(&self.matrix.column(b)));
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

	pub fn mrmr_features(&self)->Vec<(String,f64)>{
		let features = self.get_headers().clone().into_iter().filter(|f| f!="class").collect::<Vec<_>>();
		
		let mut relevances =features.iter().map(|f|(f.to_string(),0.0)).collect::<HashMap<_,_>>();
		let mut redundances =features.iter().map(|f|(f.to_string(),0.0)).collect::<HashMap<_,_>>();

		for feature in features.iter() {
			*relevances.get_mut(feature).unwrap() = self.mutual_info(feature, "class").unwrap();
		}

		// TODO: Use binary heap to keep hightest score feature
		
		let most_relevant = relevances.iter()
		.map(|(f,v)|(f.to_string(),*v))
		.reduce(
			|(acc,acc_val),(item,item_val)| if item_val > acc_val {(item,item_val)}else{(acc,acc_val)}).unwrap();
		
		let mut selected_features = vec![most_relevant];
		let mut remaining_features = features.clone().into_iter().filter(|f| f!=&selected_features.last().unwrap().0).collect::<Vec<_>>();


		while !remaining_features.is_empty(){

			for feature in remaining_features.iter(){
				*redundances.get_mut(feature).unwrap()+= self.mutual_info(feature, &selected_features.last().unwrap().0).unwrap();
			};
			let most_mrmr = remaining_features.iter()
			.map(|f|(f.to_string(),relevances[f] - (redundances[f]/selected_features.len() as f64)))
			.reduce(
				|(acc,acc_val),(item,item_val)| if item_val > acc_val {(item,item_val)}else{(acc,acc_val)}).unwrap();
			
			remaining_features = remaining_features.into_iter().filter(|f| f!=&most_mrmr.0).collect::<Vec<_>>();
			selected_features.push(most_mrmr);
		}

		return selected_features;
	}

	pub fn save(&self,path: &Path) ->Result<(), Box<dyn Error>>{
		let content = serde_json::to_string(self).unwrap();
		let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&path)?;
		Ok(file.write_all(&content.as_bytes())?)
	}

	pub fn from(path: &Path) -> Result<Self, Box<dyn Error>>{
        let mut file =  File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
		return Ok(serde_json::from_str(&content)?);
	}

	pub fn merge(self,to_merge: Dataset) -> Self{

		let instances = self.get_instances() + to_merge.get_instances();

		// BTreeSet does not preserve insertion order; it orders the string alphabetically
		let headers = self.get_headers().clone().into_iter()
			.chain(to_merge.get_headers().clone().into_iter()).collect::<BTreeSet<_>>()
			.into_iter().collect::<Vec<_>>();


		let mut subheaders_map = HashMap::new();
		let self_subheaders = self.get_subheaders();
		let to_merge_subheaders = to_merge.get_subheaders();

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
		
		// let mut matrix = IMatrix::from_iter(flat_subheaders.len(),flat_subheaders.len(),
		// for (i,sub_feature_a) in flat_subheaders.iter().enumerate(){
		// 	for (j,sub_feature_b) in flat_subheaders.iter().enumerate(){
		// 		let value  = 
		// 		*matrix.get_mut((i,j)).unwrap()=value;
		// 	}
		// }
		let subheaders_iter = flat_subheaders.iter().
			flat_map(|subheader_a| flat_subheaders.iter().map(move |subheader_b| (subheader_a,subheader_b)))
			.map(|(subheader_a,subheader_b)|{
				self.intersection(subheader_a, subheader_b).unwrap_or(0)
					+ to_merge.intersection(subheader_a, subheader_b).unwrap_or(0)
			});
		let num_subheaders = flat_subheaders.len();
		let matrix = IMatrix::from_iterator(num_subheaders, num_subheaders,subheaders_iter);
		// let matrix = CscMatrix::from(&matrix);

		Dataset{
			headers,
			subheaders:subheaders_map,
			positions,
			instances,
			matrix
		}
	}

	pub fn merge_vec(self,to_merge_vec: Vec<Dataset>) -> Self{
		let mut result = self;
		for to_merge in to_merge_vec{
			result = result.merge(to_merge)
		};
		return result;
	}


	pub fn get_headers(&self) -> &Vec<String>{
		&self.headers
	}
	pub fn get_instances(&self) -> usize{
		self.instances
	}
	pub fn get_matrix(&self) ->&IMatrix{
		&self.matrix
	}
	pub fn get_subheaders(&self) ->&HashMap<String,Vec<String>>{
		&self.subheaders
	}
	pub fn get_header_values(&self,header:&str)->Option<&Vec<String>>{
		self.get_subheaders().get(header)
	}
}
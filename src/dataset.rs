extern crate nalgebra as na;

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io;
use csv::Reader;
use na::{Dynamic, OMatrix};


type IMatrix = OMatrix<usize, Dynamic, Dynamic>;

//TODO: change sub_headers to HashMap<String,Vec<String>>
pub struct Dataset{
	headers:Vec<String>,
	sub_headers:HashMap<String,Vec<String>>,
	positions:HashMap<String,usize>,
	instances:usize,
	matrix:IMatrix
}

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
				.collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();

			
			let mut current_onehot:Vec<_> = unique_values.clone().iter()
				.flat_map(|&value | 
					instances.iter().map(move |i| if i.get(index).unwrap() == value {1}else{0}))
				.collect();
			
			onehot.append(&mut current_onehot);

			sub_headers.extend(unique_values.iter().map(|subheader|format!("{header}_{subheader}")));

			sub_headers_map.insert(header.to_string(),unique_values.iter().map(|subheader|format!("{header}_{subheader}")).collect());
			
		}
		
		
		let matrix = IMatrix::from_vec(instances.len(),sub_headers.len(),onehot);
		//Intersection of features is the product of A' * A		
		let result = matrix.tr_mul(&matrix);
		let positions = sub_headers.iter().enumerate()
			.map(|(index,value)| (value.to_string(),index))
			.collect::<HashMap<_,_>>();


		return Ok(Dataset{
			headers,
			sub_headers:sub_headers_map,
			positions,
			instances:instances.len(),
			matrix:result,
		})
	}
	pub fn intersection(&self, sub_feature_a: &str, sub_feature_b: &str)->Option<usize>{
		let cell = match (self.positions.get(sub_feature_a),self.positions.get(sub_feature_b)){
			(Some(index_a),Some(index_b))=>(*index_a,*index_b),
			_=>return None,
		};

		// return Some(self.matrix.column(a).dot(&self.matrix.column(b)));
		return self.matrix.get(cell).cloned();
	}

	pub fn mutual_info(&self,feature_a: &str, feature_b: &str) -> Option<f64>{

		let sub_headers = &self.sub_headers;
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

	pub fn get_headers(&self) -> &Vec<String>{
		&self.headers
	}
}
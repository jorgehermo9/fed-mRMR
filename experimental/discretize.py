#! /usr/bin/python3
from sklearn.preprocessing import KBinsDiscretizer
import pandas as pd
import numpy as np
import argparse

'''
Examples of usage:

./discretize.py -c ~/datasets/ObesityDataSet_raw_and_data_sinthetic.csv -b 20 -f Height Weight Age
./discretize.py -c ~/datasets/ObesityDataSet_raw_and_data_sinthetic.csv -b 10 10 10 -f Height Weight Age
./discretize.py -c ~/datasets/iris.data -b 10 -i class
'''
def parse_args():
	parser = argparse.ArgumentParser(description="csv discretization tool")
	parser.add_argument("-csv",type=str,help="csv file to discretize",required=True)
	parser.add_argument("-features",nargs="+",type=str,help="features to discretize (if no specified, all will be discretized)")
	parser.add_argument("-ignore-features",nargs="+",type=str,help=
		"features to ignore if all features select (i.e, no features arg specified). if features are specified,\
			this arg is ignored")
	parser.add_argument("-bins",nargs="+",type=int,required=True,
	help="bins to use for discretization (if only one specified, its value will be used for all features)")
	parser.add_argument("-encode",default="ordinal",const="ordinal",
		nargs="?",choices=["ordinal", "onehot","onehot-dense"],
		help="method used to encode the transformed result (default ordinal)")
	parser.add_argument("-strategy",default="quantile",const="quantile",
		nargs="?",choices=["quantile", "uniform","kmeans"],
		help="strategy used to define the widths of the bins (default quantile)")
	
	return parser.parse_args()
if __name__ == "__main__":
	parser = parse_args()
	
	df = pd.read_csv(parser.csv)
	columns = df.columns

	if parser.features:
		target_index = [index for index,column in enumerate(columns) if column in parser.features]
	elif parser.ignore_features:
		target_index = [index for index,column in enumerate(columns) if column not in parser.ignore_features]
	else:
		target_index = range(len(columns))
	
	if(len(parser.bins) != 1 and len(parser.bins) < len(target_index)):
		print("ERROR: bins' length must be 1 or match the features size")
		exit(1)

	if len(parser.bins) ==1:
		target_bins = [parser.bins[0] for _ in range(len(target_index))]
	else:
		target_bins = parser.bins

	target = df.values[:,target_index]
	est = KBinsDiscretizer(n_bins=np.array(target_bins),encode=parser.encode,strategy=parser.strategy)
	est.fit(target)
	Xt = est.transform(target)

	for no,ind in enumerate(target_index):
		df.iloc[:,ind] = Xt[:,no]

	df.iloc[:,target_index] = df.iloc[:,target_index].apply(pd.to_numeric, downcast="integer")

	df.to_csv(f"{parser.csv}.disc",index=False)


	discretized =[columns[index] for index in target_index] 
	print(f"features {discretized} discretized and file saved to {parser.csv}.disc")
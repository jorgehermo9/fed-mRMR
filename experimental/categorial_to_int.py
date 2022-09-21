#!/bin/python3
import pandas as pd
import sys
import argparse

def parse_args():
	parser = argparse.ArgumentParser(description="csv discretization tool")
	parser.add_argument("csv",type=str,help="csv ")
	parser.add_argument("-features",nargs="+",type=str,help="features")
	
	return parser.parse_args()
# Discretize dataframe columns to integers
parser = parse_args()

df = pd.read_csv(parser.csv)
target_features = parser.features if parser.features else df.columns

df[target_features] = df[target_features].astype("category")

for feature in target_features:
	df[feature] = df[feature].cat.codes

df.to_csv(f"{parser.csv}.int",index=False)	


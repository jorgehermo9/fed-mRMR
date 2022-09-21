#!/bin/python3
import pandas as pd
import sys
import argparse

def parse_args():
	parser = argparse.ArgumentParser(description="csv discretization tool")
	parser.add_argument("csv",type=str,help="csv ")
	parser.add_argument("-feature",type=str,help="features")
	parser.add_argument("-position",type=int,help="position")
	return parser.parse_args()
# Discretize dataframe columns to integers
parser = parse_args()

df = pd.read_csv(parser.csv)
rest_columns = [column for column in df.columns if column != parser.feature]
columns_titles = [parser.feature]+rest_columns
df=df.reindex(columns=columns_titles)
df.to_csv(f"{parser.csv}.switched",index=False)	


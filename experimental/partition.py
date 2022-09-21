#!/bin/python3
from pathlib import Path
import pandas as pd
import sys 
import os
"""
Example

python3 partition.py datasets/mnist_train.csv.disc 40 partitions/mnist/40
"""
csv = sys.argv[1]
csv_name = Path(csv).name

partitions = int(sys.argv[2])
dest_folder = sys.argv[3]

df = pd.read_csv(csv)

partitions_df = [df.iloc[i::partitions] for i in range(partitions)]
# Create directory dest_folder and dont fail if exists
Path(dest_folder).mkdir(parents=True, exist_ok=True)

for i in range(partitions):
	partitions_df[i].to_csv(dest_folder+f"/{csv_name}.{i}",index=False)


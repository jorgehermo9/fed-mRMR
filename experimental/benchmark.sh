#! /bin/sh

echo "Starting benchmark"
# Comparison matrix and selection for increasing number of features to select

## Colon dataset
echo "Matrix generation and feature selection increasing # of features to selenct for colon dataset"
hyperfine \
"cargo run -r -- matrix -o matrix.mrmr ../datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 64" \
"cargo run -r -- matrix -o matrix.mrmr ../datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 128" \
"cargo run -r -- matrix -o matrix.mrmr ../datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 256" \
"cargo run -r -- matrix -o matrix.mrmr ../datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 512" \
"cargo run -r -- matrix -o matrix.mrmr ../datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 1024" \
"cargo run -r -- matrix -o matrix.mrmr ../datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
--export-json benchmark_colon_increasing_features.json

python3 plot_grouped_bars.py benchmark_colon_increasing_features.json --labels 64,128,256,512,1024,2001 \
--first matrix --second "selection" -x "Number of features" -y "Time (s)" \
-o benchmark_colon_increasing_features.pdf

#

# Matrix and selection of all features for desired datasets
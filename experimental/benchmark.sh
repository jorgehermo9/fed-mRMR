#! /bin/bash

echo "Starting benchmark"
# Comparison matrix and selection for increasing number of features to select

## Colon dataset
echo "Matrix generation and feature selection increasing # of features to select for colon dataset"
hyperfine \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 64" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 128" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 256" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 512" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 1024" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
--export-json benchmark_colon_increasing_features.json

python3 plot_grouped_bars.py benchmark_colon_increasing_features.json --labels 64,128,256,512,1024,2001 \
--first "matrix calculation" --second "feature selection" -x "Number of features" -y "Time (s)" \
--title "Colon Dataset" -o benchmark_colon_increasing_features.pdf

# lymphoma dataset
echo "Matrix generation and feature selection increasing # of features to select for Lymphoma dataset"
hyperfine \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_lymphoma_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 128" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_lymphoma_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 256" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_lymphoma_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 512" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_lymphoma_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 1024" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_lymphoma_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class -n 2048" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_lymphoma_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
--export-json benchmark_lymphoma_increasing_features.json --runs 5

python3 plot_grouped_bars.py benchmark_lymphoma_increasing_features.json --labels 128,256,512,1024,2048,4027 \
--first "matrix calculation" --second "feature selection" -x "Number of features" -y "Time (s)" \
--title "Lymphoma Dataset" -o benchmark_lymphoma_increasing_features.pdf

# Matrix and selection of all features for all datasets
hyperfine \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_lung_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_lymphoma_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
"cargo run -r -- matrix -o matrix.mrmr datasets/letter-recognition.data" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
"cargo run -r -- matrix -o matrix.mrmr datasets/connect-4.data" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
"cargo run -r -- matrix -o matrix.mrmr datasets/mnist_train.csv.disc" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
--export-json benchmark_all_datasets.json

python3 plot_grouped_bars.py benchmark_all_datasets.json --labels "Lung,Colon,Lymphoma,Letter-recognition,Connect-4,MNIST" \
--first "matrix calculation" --second "feature selection" -x "Dataset" -y "Time (s)" \
--title "All Datasets" -o benchmark_all_datasets.pdf


# Comparison with original proposal

hyperfine \
"cargo run -r -- mrmr -c class --csv datasets/test_lung_s3.csv -n 300" \
"./mrmr -i datasets/test_lung_s3.csv -n 300" \
"cargo run -r -- mrmr -c class --csv datasets/test_colon_s3.csv -n 300" \
"./mrmr -i datasets/test_colon_s3.csv -n 300" \
"cargo run -r -- mrmr -c class --csv datasets/test_lymphoma_s3.csv -n 300" \
"./mrmr -i datasets/test_lymphoma_s3.csv -n 300" \
--export-json comparison_original.json --runs 5

python3 plot_grouped_bars.py comparison_original.json --labels "Lung,Colon,Lymphoma" \
--first "fed-mrmr" --second "mRMR" -x "Dataset" -y "Time (s)" \
--title "fed-mrmr versus mRMR" -o comparison_original.pdf

# Comparison with original proposal (only selection)

cargo run -r -- matrix datasets/test_lung_s3.csv -o test_lung_s3.mrmr
cargo run -r -- matrix datasets/test_colon_s3.csv -o test_colon_s3.mrmr
cargo run -r -- matrix datasets/test_lymphoma_s3.csv -o test_lymphoma_s3.mrmr

hyperfine \
"cargo run -r -- mrmr -c class test_lung_s3.mrmr -n 300" \
"./mrmr -i datasets/test_lung_s3.csv -n 300" \
"cargo run -r -- mrmr -c class test_colon_s3.mrmr -n 300" \
"./mrmr -i datasets/test_colon_s3.csv -n 300" \
"cargo run -r -- mrmr -c class test_lymphoma_s3.mrmr -n 300" \
"./mrmr -i datasets/test_lymphoma_s3.csv -n 300" \
--export-json comparison_original_selection.json --runs 5

python3 plot_grouped_bars.py comparison_original_selection.json --labels "Lung,Colon,Lymphoma" \
--first "fed-mrmr selection" --second "mRMR" -x "Dataset" -y "Time (s)" \
--title "fed-mrmr selection versus mRMR" -o comparison_original_selection.pdf

# Federated

## create Mnist partitions
# for i in 2 4 8 16 32 64; do
# 	python3 partition.py datasets/mnist_train.csv.disc $i partitions/mnist/$i
# done

# for i in 2 4 8 16 32 64; do
#  for j in $(seq 0 $(($i-1))); do
# 	cargo run -r -- matrix -o partitions/mnist/$i/matrix.mrmr.$j partitions/mnist/$i/mnist_train.csv.disc.$j
#   done
# done


echo "Benchmarking federated learning simulation"
hyperfine \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/2/*.mrmr*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/4/*.mrmr*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/8/*.mrmr*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/16/*.mrmr*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/32/*.mrmr*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/64/*.mrmr*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
--export-json benchmark_mnist_federated_increasing_nodes.json --runs 5

python3 plot_grouped_bars.py benchmark_mnist_federated_increasing_nodes.json --labels 2,4,8,16,32,64 \
--first "matrix merge" --second "feature selection" -x "Number of nodes" -y "Time (s)" \
--title "MNIST Dataset" -o benchmark_mnist_federated_increasing_nodes.pdf

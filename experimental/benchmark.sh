#! /bin/bash

mkdir partitions &>/dev/null
mkdir results &>/dev/null
mkdir output &>/dev/null
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
--export-json results/benchmark_colon_increasing_features.json

python3 plot_grouped.py results/benchmark_colon_increasing_features.json --labels 64,128,256,512,1024,2001 \
--benches "matrix calculation,feature selection" -x "Number of features" -y "Time (s)" \
--title "Colon Dataset" --sum --sum-label "total fed-mRMR" --small 30 --big 34 \
-o output/benchmark_colon_increasing_features.pdf

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
--export-json results/benchmark_lymphoma_increasing_features.json

python3 plot_grouped.py results/benchmark_lymphoma_increasing_features.json --labels 128,256,512,1024,2048,4027 \
--benches "matrix calculation,feature selection" -x "Number of features" -y "Time (s)" \
--title "Lymphoma Dataset" --sum --sum-label "total fed-mRMR" --small 24 --big 34 \
-o output/benchmark_lymphoma_increasing_features.pdf

# Matrix and selection of all features for all datasets
hyperfine \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_lung_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_colon_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
"cargo run -r -- matrix -o matrix.mrmr datasets/test_lymphoma_s3.csv" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
"cargo run -r -- matrix -o matrix.mrmr datasets/letter-recognition.data.int" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
"cargo run -r -- matrix -o matrix.mrmr datasets/connect-4.data.int.switched" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
"cargo run -r -- matrix -o matrix.mrmr datasets/mnist_train.csv.disc.int" \
"cargo run -r -- mrmr matrix.mrmr -c class" \
--export-json results/benchmark_all_datasets.json

python3 plot_grouped.py results/benchmark_all_datasets.json --labels "Lung,Colon,Lymph.,Letter-recog.,Connect-4,MNIST" \
--benches "matrix,feature selection" -x "Dataset" -y "Time (s)" \
--title "All Datasets" --sum --sum-label "total fed-mRMR" --table

# Comparison with original proposal
hyperfine \
"cargo run -r -- matrix datasets/test_lung_s3.csv -o test_lung_s3.mrmr" \
"cargo run -r -- mrmr -c class test_lung_s3.mrmr -n 300" \
"./mrmr -i datasets/test_lung_s3.csv -n 300" \
"cargo run -r -- matrix datasets/test_colon_s3.csv -o test_colon_s3.mrmr" \
"cargo run -r -- mrmr -c class test_colon_s3.mrmr -n 300" \
"./mrmr -i datasets/test_colon_s3.csv -n 300" \
"cargo run -r -- matrix datasets/test_lymphoma_s3.csv -o test_lymphoma_s3.mrmr" \
"cargo run -r -- mrmr -c class test_lymphoma_s3.mrmr -n 300" \
"./mrmr -i datasets/test_lymphoma_s3.csv -n 300" \
"cargo run -r -- matrix datasets/letter-recognition.data.int -o letter-recognition.mrmr" \
"cargo run -r -- mrmr -c class letter-recognition.mrmr -n 16" \
"./mrmr -i datasets/letter-recognition.data.int -n 16 -s 21000" \
"cargo run -r -- matrix datasets/connect-4.data.int.switched -o connect-4.mrmr" \
"cargo run -r -- mrmr -c class connect-4.mrmr -n 42" \
"./mrmr -i datasets/connect-4.data.int.switched -n 42 -s 68000" \
"cargo run -r -- matrix datasets/mnist_train.csv.disc.int -o mnist.mrmr" \
"cargo run -r -- mrmr -c class mnist.mrmr -n 32" \
"./mrmr -i datasets/mnist_train.csv.disc.int -n 32 -s 43000" \
--export-json results/benchmark_comparison_original.json --runs 5

python3 plot_grouped.py results/benchmark_comparison_original.json  \
--benches "matrix,selection,mRMR" --labels "Lung,Colon,Lymph.,Letter-recog.,Connect-4,MNIST" \
--title "fed-mrmr versus mRMR" -x "Dataset" -y "Time (s)" --sum --sum-label "fed-mRMR" \
--table

# Comparison with original proposal increasing number of features
hyperfine \
"cargo run -r -- matrix -o matrix.mrmr datasets/mnist_train.csv.disc.int" \
"cargo run -r -- mrmr -c class -n 2 matrix.mrmr" \
"./mrmr -i datasets/mnist_train.csv.disc.int -n 2 -s 43000" \
"cargo run -r -- matrix -o matrix.mrmr datasets/mnist_train.csv.disc.int" \
"cargo run -r -- mrmr -c class -n 4 matrix.mrmr" \
"./mrmr -i datasets/mnist_train.csv.disc.int -n 4 -s 43000" \
"cargo run -r -- matrix -o matrix.mrmr datasets/mnist_train.csv.disc.int" \
"cargo run -r -- mrmr -c class -n 8 matrix.mrmr" \
"./mrmr -i datasets/mnist_train.csv.disc.int -n 8 -s 43000" \
"cargo run -r -- matrix -o matrix.mrmr datasets/mnist_train.csv.disc.int" \
"cargo run -r -- mrmr -c class -n 16 matrix.mrmr" \
"./mrmr -i datasets/mnist_train.csv.disc.int -n 16 -s 43000" \
"cargo run -r -- matrix -o matrix.mrmr datasets/mnist_train.csv.disc.int" \
"cargo run -r -- mrmr -c class -n 24 matrix.mrmr" \
"./mrmr -i datasets/mnist_train.csv.disc.int -n 24 -s 43000" \
"cargo run -r -- matrix -o matrix.mrmr datasets/mnist_train.csv.disc.int" \
"cargo run -r -- mrmr -c class -n 32 matrix.mrmr" \
"./mrmr -i datasets/mnist_train.csv.disc.int -n 32 -s 43000" \
--export-json results/benchmark_comparison_increasing_features.json --runs 5

python3 plot_grouped.py results/benchmark_comparison_increasing_features.json  \
--benches "fed-mRMR matrix,fed-mRMR selection,mRMR" --labels "2,4,8,16,24,32" --title "fed-mRMR versus mRMR increasing number of features" \
-x "Number of features" -y "Time (s)" --sum --sum-label "total fed-mRMR" --small 24 --big 34 \
-o output/benchmark_comparison_increasing_features.pdf

# Federated

# create Mnist partitions
for i in 1 2 4 8 16 32 64 512 1024; do
	python3 partition.py datasets/mnist_train.csv.disc.int $i partitions/mnist/$i
done

for i in 1 2 4 8 16 32 64 512 1024; do
	hyperfine --parameter-scan partition 0 $(($i-1)) \
	"cargo run -r -- matrix -o partitions/mnist/$i/matrix.mrmr.{partition} partitions/mnist/$i/mnist_train.csv.disc.int.{partition}"\
	--export-json results/partitions/benchmark_federated_mnist_matrix_partition_$i.json --runs 2
done

echo "Benchmarking federated learning simulation"
hyperfine \
"/bin/true" \
"cargo run -r -- mrmr partitions/mnist/1/matrix.mrmr.0 -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/2/*.mrmr.*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/4/*.mrmr.*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/8/*.mrmr.*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/16/*.mrmr.*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/32/*.mrmr.*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/64/*.mrmr.*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/512/*.mrmr.*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
"cargo run -r -- merge -o merged.mrmr partitions/mnist/1024/*.mrmr.*" \
"cargo run -r -- mrmr merged.mrmr -c class" \
--export-json results/benchmark_mnist_federated_increasing_nodes.json --runs 2

python3 plot_grouped.py results/benchmark_mnist_federated_increasing_nodes.json \
--benches "matrix merge,feature selection" --labels "1,2,8,32,64,512,1024" --title "MNIST dataset" \
-x "Number of nodes" -y "Time (s)" --sum --sum-label "total fed-mRMR" --federated \
-o output/benchmark_mnist_federated_increasing_nodes.pdf

rm *.mrmr
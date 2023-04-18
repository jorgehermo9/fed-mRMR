# Building project

To compile project, execute:

```console
cargo build --release
```

> Binary file is created in ./target/release/fed-mrmr, alternatively you can run with `cargo run --release`
> Run with --help flag to show general help, and with a subcommand to show subcommand's help

Available subcommands:
- matrix
- merge
- mrmr
- show

> Example datasets can be found under `test/assets` folder

# Create occurrences matrix

From a given csv dataset:

```console
./target/release/fed-mrmr matrix -o matrix.mrmr <dataset.csv>
```
Occurrence matrix is serialized and saved to `matrix.mrmr` file in JSON format.

# Show occurrences matrix

From a previously serialized occurrences matrix:

```console
./target/release/fed-mrmr show matrix.mrmr
```
> Note: matrix prints to stdout, so it may overflow terminal's columns, you may want to better redirect stdout to a file.

# Select features (mRMR)

From a previously serialized occurrences matrix:

```console
./target/release/fed-mrmr mrmr -c class matrix.mrmr
```

Alternatively, you can select features without a serialized mrmr matrix, by providing the csv dataset as input:

```console
./target/release/fed-mrmr mrmr -c class --csv <dataset.csv>
```
By using flag `-v` or `-vv`, more info about ranking is shown. 

> Note: is important to specify class name with -c argument.

# Merge occurrences matrix into a single matrix

From N previously serialized occurrences matrix:

```console
./target/release/fed-mrmr merge -o merged.mrmr matrix1.mrmr matrix2.mrmr matrixn.mrmr
```

# Examples

Example using a mock dataset, located in `test/assets/dataset.csv`
```console
# Generate occurrences matrix
./target/release/fed-mrmr matrix -o matrix.mrmr test/assets/dataset.csv

# Show occurrences matrix
./target/release/fed-mrmr show matrix.mrmr

# Select features
./target/release/fed-mrmr mrmr -c class matrix.mrmr -vv
```

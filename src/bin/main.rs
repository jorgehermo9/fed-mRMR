use clap::{Parser, Subcommand};
use csv::Reader;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use std::{error::Error, path::PathBuf};

use fed_mrmr::dataset::*;

#[derive(Parser)]
#[clap(author,version,about,long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Mrmr {
        /// read file from provided path. If none is provided, file is read from stdin
        path: Option<PathBuf>,
        /// flag to try to parse input file as csv format
        #[clap(long)]
        csv: bool,
        /// specifiy class feature name of the dataset
        #[clap(short, long)]
        class: String,
        /// max number of features to select
        #[clap(short, long)]
        num_features: Option<usize>,

        /// display more output info. -v for ranking order info and -vv for ranking values and computation times
        #[clap(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
    },
    Matrix {
        /// read file from provided path. If none is provided, file is read from stdin
        path: Option<PathBuf>,
        /// path to write matrix
        #[clap(short, long)]
        output: PathBuf,
    },
    Merge {
        /// datasets to merge
        #[clap(required = true, min_values = 2)]
        datasets: Vec<PathBuf>,

        /// path to write matrix
        #[clap(short, long)]
        output: PathBuf,
    },
    Show {
        /// read file from provided path. If none is provided, file is read from stdin
        path: Option<PathBuf>,
    },
}

fn mrmr(
    path: &Option<PathBuf>,
    csv: bool,
    class: &String,
    limit: &Option<usize>,
    verbose: u8,
) -> Result<(), Box<dyn Error>> {
    let reader: Box<dyn BufRead> = match path {
        Some(path) => Box::new(BufReader::new(fs::File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };
    let dataset = if csv {
        //If csv flag specified and no path is provided, read csv from stdin
        let start_matrix = Instant::now();
        let dataset = Dataset::new(Reader::from_reader(reader))?;
        if verbose >= 2 {
            let duration_matrix = start_matrix.elapsed();
            println!(
                "Elapsed time for matrix construction: {}s",
                duration_matrix.as_secs_f32()
            );
        }
        dataset
    } else {
        Dataset::from_reader(reader)?
    };

    let start_mrmr = Instant::now();
    let selected_features = dataset.mrmr_features(class, *limit);

    let feature_padding = dataset.get_headers().iter().map(|s| s.len()).max().unwrap();
    //+2 because dot and first digit
    let rank_padding = (selected_features.len() as f32).log10() as usize + 2;
    for (index, (feature, value)) in selected_features.into_iter().enumerate() {
        if verbose == 0 {
            print!("{feature} ")
        } else if verbose == 1 {
            let rank = format!("{}.", index + 1);
            println!("{rank:<rank_padding$} {feature:<feature_padding$}");
        } else if verbose >= 2 {
            let rank = format!("{}.", index + 1);
            println!("{rank:<rank_padding$} {feature:<feature_padding$} -> {value:.6}");
        }
    }
    if verbose == 0 {
        println!();
    } else if verbose >= 2 {
        let duration_mrmr = start_mrmr.elapsed();
        println!(
            "Elapsed time for mrmr calculation: {}s",
            duration_mrmr.as_secs_f32()
        );
    }

    Ok(())
}

fn matrix(path: &Option<PathBuf>, output: &PathBuf) -> Result<(), Box<dyn Error>> {
    let reader: Box<dyn BufRead> = match path {
        Some(path) => Box::new(BufReader::new(fs::File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    //TODO: display matrix construction elapsed time with verbose flag
    let dataset = Dataset::new(Reader::from_reader(reader))?;
    dataset.save(output)?;
    println!("Matrix saved to {}", output.display());

    Ok(())
}

fn merge(datasets: &Vec<PathBuf>, output: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut paths = datasets.iter();
    let first_path = paths.next().unwrap();
    let mut result = Dataset::from_path(first_path)?;
    for path in paths {
        let dataset = Dataset::from_path(path)?;
        result = result.merge(dataset);
    }
    result.save(output)?;
    println!("Merged dataset matrix saved to {}", output.display());

    Ok(())
}

fn show(path: &Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    let reader: Box<dyn BufRead> = match path {
        Some(path) => Box::new(BufReader::new(fs::File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };
    let dataset = Dataset::from_reader(reader)?;

    let features = dataset.get_headers();
    let sub_features_map = dataset.get_subheaders();
    let flat_sub_features = features
        .iter()
        .flat_map(|feature| sub_features_map.get(feature).unwrap());
    println!(
        "{} sub_features, {} instances\n",
        flat_sub_features.clone().count(),
        dataset.get_instances()
    );

    for sub_feature in flat_sub_features {
        print!("{sub_feature} ");
    }
    println!();

    let matrix = dataset.get_matrix();
    println!("{matrix}");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Mrmr {
            path,
            csv,
            class,
            num_features,
            verbose,
        } => mrmr(path, *csv, class, num_features, *verbose)?,
        Commands::Matrix { path, output } => matrix(path, output)?,
        Commands::Merge { datasets, output } => merge(datasets, output)?,
        Commands::Show { path } => show(path)?,
    };

    Ok(())
}

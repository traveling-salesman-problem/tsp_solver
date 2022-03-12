use std::fs::File;
use std::fs::remove_file;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use clap::Parser;

mod dataset;
mod individual;
mod generation;
mod utils;

use dataset::Dataset;
use generation::Generation;
use utils::ThousandsDisplayPolicy;

// create a command line arguments parser
#[derive(Parser)]
#[clap(author, version, about)]
struct ArgsParser {
  // dataset filename
  #[clap(short='d', long, default_value="datasets/demo/demo.json", help="The url of the dataset (in JSON format)")]
  dataset_filename: String,
  
  // logs filename
  #[clap(short='l', long, default_value="logs.txt", help="The url of the file to log everything to")]
  logs_filename: String,

  // number of generations
  #[clap(short='g', long, default_value="10", help="The number of generations to run")]
  number_of_generations: usize,

  // population size
  #[clap(short='p', long, default_value="200", help="The number of individuals in each generation")]
  population_size: usize,

  // neighbors distance lookup
  #[clap(short='n', long, default_value="4", help="The number of neighbors to look up : MUST NOT BE BIGGER THAN THE NUMBER OF POINTS IN THE DATASET !")]
  neighbors_distance_lookup: usize,

  // best out of
  #[clap(short='b', long, default_value="10", help="The number of children generated during the mutation process of one individual : We only keep the best out of this number of children")]
  best_out_of: usize,

  // display interval
  #[clap(short='i', long, default_value="20", help="The number of generations between each display (if the number is too small it will slow down the algorithm)")]
  display_interval: usize,

  // generations logging
  // #[clap(short='L', long, default_value=True, help="Whether to log each generation")]
  // log_generations: bool,
}

// compute the factorial of a number
fn floaty_factorial(n: usize) -> f64 {
  let mut result = 1.0;
  for i in 1..n+1 {
    result *= i as f64;
  }
  result
}

// entry of the program
fn main() {
  // parse the command line arguments
  let args = ArgsParser::parse();

  // load the dataset into RAM
  let dataset = Dataset::from_file(&args.dataset_filename);

  // log the number of valid solutions to the dataset
  println!("{}! = {:.3e} valid solutions to the dataset", dataset.size, floaty_factorial(dataset.size));

  // reset the logs
  if (Path::new(&args.logs_filename)).exists() {
    remove_file(&args.logs_filename).expect("Unable to remove the log file");
  }
  let mut log_file = File::create(&args.logs_filename).expect("Unable to create the log file");

  // log the dataset
  write!(log_file, "{}\n", dataset).expect("Unable to write to the log file");
  
  // create a random number generator
  let mut rng = rand::thread_rng();

  // start stopwatch
  let stopwatch = Instant::now();
  
  // create a generation & log it
  let mut generation = Generation::new(1, args.number_of_generations, args.population_size, &dataset, &mut rng);
  // write!(log_file, "{}\n", generation).expect("Unable to write to the log file");

  // evolve through generations
  for _ in 1..args.number_of_generations {
    generation = generation.evolve(&mut rng, args.neighbors_distance_lookup, args.best_out_of);
    // if generation.id % args.display_interval == 0 {
      // write!(log_file, "{}\n", generation).expect("Unable to write to the log file");
    // }
  }

  // stop stopwatch
  let execution_duration = stopwatch.elapsed();
  println!("search time : {}s\n", (execution_duration.as_millis() as f64 / 1000.0).thousands());

  // display the best solution
  let mut best_solution = String::new();

  best_solution.push_str(&format!("┌─ BEST SOLUTION {:─>gen_padding$}─┐\n", "", gen_padding=generation.population[0].individual_display_width-15).as_str());
  best_solution.push_str(&format!("│ {} │\n", generation.population[0]));
  best_solution.push_str(&format!("└─{:─>gen_padding$}─┘\n", "", gen_padding=generation.population[0].individual_display_width));

  write!(log_file, "{}", best_solution).expect("Unable to write to the log file");
  println!("{}", best_solution);
}

// fn box_stringable(data: Box<dyn ToString>) -> String {
//   box_array(vec![data])
// }

// fn box_array(data: Vec<Box<dyn ToString>>) -> String {
//   let mut result = String::new();
//   let mut rows_as_strings: Vec<String> = Vec::new();
//   for row in data {
//     rows_as_strings.push(row.to_string());
//   }
//   let max_row_length = rows_as_strings.iter().fold(0, |current_max, row| current_max.max(row.len()));
//   result.push_str("hello\n");
//   for string in rows_as_strings {
//     result.push_str(&string);
//   }
//   result.push_str("world\n");
//   result
// }
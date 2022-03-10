use rand::prelude::*;
use std::fmt::Display;
use thousands::Separable;

use crate::solution::Solution;
use crate::points_dataset::PointsDataset;

// define the Generation struct
pub struct Generation<'a> {
  pub generation_number: usize,
  pub number_of_generations: usize,
  pub number_of_generations_display_width: usize,
  pub population_size: usize,
  pub population: Vec<Solution<'a>>
}

// implement the Generation struct
impl<'a> Generation<'a> {
  // returns a new instance of the Generation struct
  pub fn new(generation_number: usize, number_of_generations: usize, population_size: usize, points_dataset: &'a PointsDataset, random_number_generator: &mut ThreadRng) -> Generation<'a> {
    // create a new vector of solutions
    let mut population = Vec::new();

    // create new random solutions
    for _ in 0..population_size {
      population.push(Solution::new(points_dataset, random_number_generator));
    }

    // sort the solutions by their length
    population.sort_by(|sol_1, sol_2| sol_1.partial_cmp(sol_2).expect("Unable to compare solutions while creating a new generation"));

    // create the struct
    Generation {
      generation_number,
      number_of_generations,
      number_of_generations_display_width: number_of_generations.separate_with_commas().len(),
      population_size,
      population
    }
  }

  // // returns a new empty generation
  // pub fn new_empty(generation_number: usize, population_size: usize, number_of_generations: usize) -> Generation<'a> {
  //   // create the struct
  //   Generation {
  //     generation_number,
  //     number_of_generations,
  //     number_of_generations_width: number_of_generations.separate_with_commas().len(),
  //     population_size,
  //     population: Vec::new()
  //   }
  // }

  // returns a new empty generation from a previous generation
  pub fn new_empty_from_previous(previous_generation: &Generation<'a>) -> Generation<'a> {
    Generation {
      generation_number: previous_generation.generation_number + 1,
      number_of_generations: previous_generation.number_of_generations,
      number_of_generations_display_width: previous_generation.number_of_generations_display_width,
      population_size: previous_generation.population_size,
      population: Vec::new()
    }
  }

  // create the next generation
  pub fn evolve(&self, rng: &mut ThreadRng, neighbors_distance_lookup: usize, best_out_of: usize) -> Generation<'a> {
    // create the next generation
    let mut new_generation = Generation::new_empty_from_previous(self);

    // populate the new generation
    for _ in 0..self.population_size {
      // let parent_1 = &self.population[rng.gen_range(0..self.population_size / 2)];
      // let parent_2 = &self.population[rng.gen_range(0..self.population_size / 2)];
      // let mut child = Solution::crossover(parent_1, parent_2);
      // child.mutate(rng);
      // new_generation.population.push(child);

      // select a parent
      let min_length = self.population.first().expect("Error while retrieving first solution in the population").length();
      let max_length = self.population.last().expect("Error while retrieving last solution in the population").length();
      let delta = max_length + (max_length - min_length)/2.0;
      let total = self.population.iter().fold(0f64, |acc, sol| acc + sol.length());
      let mut index = rng.gen_range(0.0..total);
      let mut parent_index = 0;
      for i in 0..self.population_size {
        let score = delta - self.population[i].length();
        if index < score {
          parent_index = i;
          break;
        } else {
          index -= score;
        }
      }

      // create a child from this parent
      let mut child = self.population[parent_index].clone();
      child = child.mutate(rng, neighbors_distance_lookup, best_out_of);

      // add the child to the new generation
      new_generation.population.push(child);
    }

    // sort the new generation by their length
    new_generation.population.sort_by(|sol_1, sol_2| sol_1.partial_cmp(sol_2).expect("Unable to compare solutions while creating a new generation"));

    // return the newly generated generation
    new_generation
  }
}

// implement the Display trait for the Generation struct
impl<'a> Display for Generation<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "┌─ GENERATION #{:0>gen_padding$} {:─>gen_padding_2$}─┐\n", self.generation_number, "", gen_padding=self.number_of_generations_display_width, gen_padding_2=self.population[0].solution_display_width-14-self.number_of_generations_display_width)?;

    for index in 0..self.population_size {
      write!(f, "│ {} │\n", self.population[index])?;
    }

    write!(f, "└─{:─>gen_padding$}─┘\n", "", gen_padding=self.population[0].solution_display_width)?;
    
    Ok(())
  }
}

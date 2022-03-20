use rand::prelude::*;
use std::fmt::Display;
use thousands::Separable;

use crate::individual::Individual;
use crate::dataset::Dataset;

// define the Generation struct
pub struct Generation<'a> {
  pub id: usize,
  pub number_of_generations: usize,
  pub number_of_generations_display_width: usize,
  pub population_size: usize,
  pub population: Vec<Individual<'a>>,
  pub fitnesses: Vec<f64>
}

// implement the Generation struct
impl<'a> Generation<'a> {
  // function that computes the fitnesses of each individual in the population
  fn compute_fitnesses(population: &Vec<Individual<'a>>, population_size: usize) -> Vec<f64> {
    // find min and max scores
    let max = population.last().expect("Unable to get last element of population").length;
    let min = population.first().expect("Unable to get first element of population").length;

    // compute the score of each individual
    let delta = max - min;
    let delta = delta * delta / population_size as f64;

    let mut fitnesses: Vec<f64> = population.iter()
      .map(|ind| max - ind.length)
      .map(|length| length*length + delta)
      .collect();
    
    // make the mapping progressive
    for index in 1..population_size {
      fitnesses[index] += fitnesses[index-1];
    }
    
    // return results scaled to [0, 1]
    let length = fitnesses[population_size-1];
    fitnesses.iter().map(|fitness| fitness/length).collect()
  }

  // returns a new instance of the Generation struct
  pub fn new(id: usize, number_of_generations: usize, population_size: usize, dataset: &'a Dataset, rng: &mut ThreadRng) -> Self {
    // create a new vector of solutions
    let mut population = Vec::new();

    // create new random solutions
    for _ in 0..population_size {
      population.push(Individual::new(dataset, rng));
    }

    // sort the solutions by their length
    population.sort_by(|ind_1, ind_2| ind_1.partial_cmp(ind_2).expect("Unable to compare individuals while creating a new generation"));

    // create the struct
    let fitnesses = Generation::compute_fitnesses(&population, population_size);
    Self {
      id,
      number_of_generations,
      number_of_generations_display_width: number_of_generations.separate_with_commas().len(),
      population,
      population_size,
      fitnesses
    }
  }

  // select a parent for crossover depending on the selection weights
  pub fn select_parent(&self, rng: &mut ThreadRng) -> &Individual<'a> {
    let pointer = rng.gen_range(0f64..1f64);
    let selected_parent_index = self.fitnesses.iter()
      .position(|&weight| pointer <= weight)
      .expect("Unable to find parent");
    &self.population[selected_parent_index]
  }

  // returns a new empty generation from a previous generation
  pub fn new_empty_from_previous(previous_generation: &Self) -> Self {
    Self {
      id: previous_generation.id + 1,
      number_of_generations: previous_generation.number_of_generations,
      number_of_generations_display_width: previous_generation.number_of_generations_display_width,
      population_size: previous_generation.population_size,
      population: Vec::new(),
      fitnesses: Vec::new()
    }
  }

  // create the next generation
  pub fn evolve(&self, rng: &mut ThreadRng, neighbors_distance_lookup: usize, best_out_of: usize) -> Self {
    // create the next generation
    let mut new_generation = Self::new_empty_from_previous(self);

    // populate the new generation
    for _ in 0..self.population_size {
      let mut child;

      if self.id >= 10 {
        child = self.select_parent(rng).mutate(rng, neighbors_distance_lookup, best_out_of);
      } else {
        // select parents
        let parent1 = self.select_parent(rng);
        let parent2 = self.select_parent(rng);
  
        // create a child from this parents
        child = Individual::crossover(parent1, parent2);
        // mutate the child
        child = child.mutate(rng, neighbors_distance_lookup, best_out_of);
      }

      // add the child to the new generation
      new_generation.population.push(child);
    }

    // sort the new generation by their length
    new_generation.population.sort_by(|sol_1, sol_2| sol_1.partial_cmp(sol_2).expect("Unable to compare solutions while creating a new generation"));

    // compute the selection weights
    new_generation.fitnesses = Generation::compute_fitnesses(
      &new_generation.population,
      new_generation.population_size
    );

    // return the newly generated generation
    new_generation
  }
}

// implement the Display trait for the Generation struct
impl<'a> Display for Generation<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "┌─ GENERATION #{:0>gen_padding$} {:─>gen_padding_2$}─┐\n", self.id, "", gen_padding=self.number_of_generations_display_width, gen_padding_2=self.population[0].individual_display_width-14-self.number_of_generations_display_width)?;

    for index in 0..self.population_size {
      write!(f, "│ {} │\n", self.population[index])?;
    }

    write!(f, "└─{:─>gen_padding$}─┘\n", "", gen_padding=self.population[0].individual_display_width)?;
    
    Ok(())
  }
}

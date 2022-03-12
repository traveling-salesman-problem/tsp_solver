use std::fmt::Display;
use std::cmp::{Ordering};
use rand::prelude::*;
use thousands::Separable;
use crate::dataset::Dataset;

// definition of the Individual struct
// it represents a valid solution to the problem
pub struct Individual<'a>  {
  pub size: usize,
  pub nodes: Vec<usize>,
  pub dataset: &'a Dataset,
  length: Option<f64>,
  pub individual_display_width: usize
}

// implement the Individual struct
impl<'a> Individual<'a> {
  // update the length of the individual
  pub fn update_length(&mut self) {
    // define a variable that will hold the total length of the individual
    let mut total_length = 0.0;

    // for each node, compute the distance to the next
    for node_index in 0..self.size - 1 {
      total_length += self.dataset.distance_matrix[self.nodes[node_index]][self.nodes[node_index + 1]];
    }
    
    // save the length for later
    self.length = Some(total_length);
  }

  // returns a new random instance of the individual struct
  pub fn new (dataset: &'a Dataset, rng: &mut ThreadRng) -> Self {
    // generate a list of nodes as numbers
    let mut nodes: Vec<usize> = (0..dataset.size).collect();

    // shuffle the order in which the nodes are visited
    nodes.shuffle(rng);

    // create the individual
    let mut individual = Self {
      size: dataset.size,
      nodes,
      dataset,
      length: None,
      individual_display_width: dataset.size * (dataset.longest_label_display_width + 4) - 1 + dataset.longest_path_display_width
    };
    individual.update_length();
    individual
  }

  // // returns an empty instance of the Solution struct
  // pub fn new_empty (nodes_dataset: &'a nodesDataset) -> Solution<'a> {
  //   Solution { nodes: vec![], nodes_dataset, length: None }
  // }

  // // returns an empty instance of the Solution struct generated from a parent solution
  // pub fn new_empty_from_parent (parent_solution: &'a Solution) -> Solution<'a> {
  //   Solution::new_empty(parent_solution.nodes_dataset)
  // }

  // retrieve the length of the individual
  pub fn length(&self) -> f64 {
    match self.length {
      Some(length) => length,
      None => panic!("The length of the individual has not been computed yet")
    }
  }

  // // returns a new instance of the Solution struct generated from two parent solutions
  // pub fn crossover(parent_1: &Solution<'a>, parent_2: &Solution<'a>) -> Solution<'a> {
  //   // create a new empty solution
  //   let mut child = Solution::new_empty_from_parent(parent_1);

  //   // crossover algorithm
  //   for i in 0..child.nodes_dataset.nodes_count {
  //     child.nodes.push(parent_1.nodes[i]);
  //   }

  //   // return the newly created child
  //   child
  // }

  // mutate randomly the individual
  pub fn mutate (&self, rng: &mut ThreadRng, neighbors_distance_lookup: usize, best_out_of: usize) -> Self {
    // we apply the mutation multiple times and only keep the best one
    let mut best_child: Option<Self> = None;

    for _ in 0..best_out_of {
      // let current individual
      let mut child = self.clone();

      // apply inversion mutation
      let mut index_1 = rng.gen_range(0..child.size);
      let mut index_2 = rng.gen_range(0..child.size);
      if index_1 > index_2 {
        let tmp = index_1;
        index_1 = index_2;
        index_2 = tmp;
      }
      for i in index_1..(index_1 + index_2 + 1)/2 {
        let temp = child.nodes[i];
        child.nodes[i] = child.nodes[index_2 - i + index_1];
        child.nodes[index_2 - i + index_1] = temp;
      }

      // apply exchange mutation
      let index_1 = rng.gen_range(0..child.size);
      let node_1 = child.nodes[index_1];
      let distance: usize = rng.gen_range(0..neighbors_distance_lookup);
      let node_2 = child.dataset.nodes_neighbors[node_1][distance];
      let index_2 = child.nodes.iter().position(|&node| node == node_2).expect("node not found during mutation exchange");

      child.nodes[index_1] = node_2;
      child.nodes[index_2] = node_1;

      // update the length
      child.update_length();

      // save the child if it is the best individual so far
      if best_child.is_none() || child.length() < best_child.as_ref().expect("Previous best individual not found").length() {
        best_child = Some(child);
      }
    }

    best_child.expect("Best individual not found")
  }
}
  
// implement comparisons operators for the Individual struct
// the comparison is based on the length of the individual
impl<'a> PartialEq for Individual<'a> {
  fn eq (&self, other: &Self) -> bool {
    self.length() == other.length()
  }
}

impl<'a> PartialOrd for Individual<'a> {
  fn partial_cmp (&self, other: &Self) -> Option<Ordering> {
    self.length().partial_cmp(&other.length())
  }
}

// implement the Clone trait for the individual struct
impl<'a> Clone for Individual<'a> {
  fn clone (&self) -> Self {
    Self {
      size: self.size,
      nodes: self.nodes.clone(),
      dataset: self.dataset,
      length: self.length,
      individual_display_width: self.individual_display_width
    }
  }
}

// implement the Display trait for the Individual struct
impl<'a> Display for Individual<'a> {
  fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // create a new string that will contain the stringified individual
    let mut result = String::from("");

    // for each node in the individual, add it to the string
    for (node_index, &node) in self.nodes.iter().enumerate() {
      result.push_str(&format!("{:>width$}", self.dataset.labels[node], width = self.dataset.longest_label_display_width));
      if node_index < self.size - 1 {
        result.push_str(" -> ");
      }
    }

    // add the length of the individual and a new line
    result.push_str(&format!(" · {:>width$}", self.length().separate_with_commas(), width = self.dataset.longest_path_display_width));

    // write the string to the formatter
    write!(f, "{}", result)
  }
}

use std::fmt::Display;
use std::cmp::{Ordering};
use rand::prelude::*;
use crate::dataset::{Dataset,Matrix};
use crate::utils::ThousandsDisplayPolicy;

// definition of the Individual struct
// it represents a valid solution to the problem
pub struct Individual<'a>  {
  pub size: usize,
  pub nodes: Vec<usize>,
  pub dataset: &'a Dataset,
  pub length: f64,
  pub individual_display_width: usize
}

// implement the Individual struct
impl<'a> Individual<'a> {
  // update the length of the individual
  fn compute_length(distance_matrix: &Matrix, nodes: &Vec<usize>, size: usize) -> f64 {
    // define a variable that will hold the total length of the individual
    let mut total_length = 0.0;

    // for each node, compute the distance to the next
    for node_index in 0..size - 1 {
      total_length += distance_matrix.get(nodes[node_index], nodes[node_index + 1]);
    }
    
    // return the computed length
    total_length
  }

  // returns a new random instance of the individual struct
  pub fn new (dataset: &'a Dataset, rng: &mut ThreadRng) -> Self {
    // generate a list of nodes as numbers
    let mut nodes: Vec<usize> = (0..dataset.size).collect();

    // shuffle the order in which the nodes are visited
    nodes.shuffle(rng);

    // compute the length of the individual
    let length = Self::compute_length(&dataset.distance_matrix, &nodes, dataset.size);

    // create the individual
    Self {
      size: dataset.size,
      nodes,
      dataset,
      length,
      individual_display_width: dataset.size * (dataset.longest_label_display_width + 4) - 1 + dataset.longest_path_display_width
    }
  }

  // returns an empty instance of the Solution struct generated from a parent solution
  pub fn new_empty_from_parent (parent: &Self) -> Self {
    Self {
      size: parent.size,
      nodes: vec![0; parent.size],
      dataset: parent.dataset,
      length: 0.0,
      individual_display_width: parent.individual_display_width
    }
  }

  // returns a new instance of the Solution struct generated from two parent solutions
  pub fn crossover(parent1: &Self, parent2: &Self, rng: &mut ThreadRng) -> Self {
    // build node map from parents
    let mut parent1_nodemap: Vec<Option<usize>> = vec![None; parent1.size];
    for (index, &node) in parent1.nodes[..parent1.size-1].iter().enumerate() {
      parent1_nodemap[node] = Some(parent1.nodes[index+1]);
    }
    let mut parent2_nodemap: Vec<Option<usize>> = vec![None; parent2.size];
    for (index, &node) in parent2.nodes[..parent2.size-1].iter().enumerate() {
      parent2_nodemap[node] = Some(parent2.nodes[index+1]);
    }

    // create a new empty solution
    let mut child = Self::new_empty_from_parent(&parent1);

    // append a first city
    child.nodes[0] = parent1.nodes[0];

    // crossover algorithm
    let mut remaining_nodes: Vec<bool> = vec![true; child.size];
    remaining_nodes[child.nodes[0]] = false;

    for i in 1..child.size {
      let last_node = child.nodes[i-1];

      let parent1_next = parent1_nodemap[last_node];
      let parent2_next = parent2_nodemap[last_node];
      
      fn find_next<'a>(child: &mut Individual<'a>, i:usize, last_node: usize, remaining_nodes: &mut Vec<bool>) {
        for &potential_next_node in child.dataset.nodes_neighbors[last_node].iter() {
          if potential_next_node != last_node {
            if remaining_nodes[potential_next_node] {
              remaining_nodes[potential_next_node] = false;
              child.nodes[i] = potential_next_node;
              return;
            }
          }
        }
      }

      fn try_set_node<'a>(child: &mut Individual<'a>, i:usize, last_node: usize, target_node: usize, remaining_nodes: &mut Vec<bool>) {
        if remaining_nodes[target_node] {
          remaining_nodes[target_node] = false;
          child.nodes[i] = target_node;
        } else {
          find_next(child, i, last_node, remaining_nodes);
        }
      }

      fn try_set_node_2<'a>(child: &mut Individual<'a>, i:usize, last_node: usize, target_node_1: usize, target_node_2: usize, remaining_nodes: &mut Vec<bool>) {
        if remaining_nodes[target_node_1] {
          remaining_nodes[target_node_1] = false;
          child.nodes[i] = target_node_1;
        } else if remaining_nodes[target_node_2] {
          remaining_nodes[target_node_2] = false;
          child.nodes[i] = target_node_2;
        } else {
          find_next(child, i, last_node, remaining_nodes);
        }
      }

      match (parent1_next, parent2_next) {
        (Some(p1_next), Some(p2_next)) => {
          if child.dataset.distance_matrix.get(last_node,p1_next) < child.dataset.distance_matrix.get(last_node,p2_next) {
            try_set_node_2(&mut child, i, last_node, p1_next, p2_next, &mut remaining_nodes);
          } else {
            try_set_node_2(&mut child, i, last_node, p2_next, p1_next, &mut remaining_nodes);
          }
        },
        (Some(p1_next), None) => try_set_node(&mut child, i, last_node, p1_next, &mut remaining_nodes),
        (None, Some(p2_next)) => try_set_node(&mut child, i, last_node, p2_next, &mut remaining_nodes),
        (None, None) => find_next(&mut child, i, last_node, &mut remaining_nodes)
      }
    }

    // update child's length
    child.length = Self::compute_length(&child.dataset.distance_matrix, &child.nodes, child.size);

    // return the newly created child
    child
  }

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
      child.length = Self::compute_length(&child.dataset.distance_matrix, &child.nodes, child.size);

      // save the child if it is the best individual so far
      if best_child.is_none() || child.length < best_child.as_ref().expect("Previous best individual not found").length {
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
    self.length == other.length
  }
}

impl<'a> PartialOrd for Individual<'a> {
  fn partial_cmp (&self, other: &Self) -> Option<Ordering> {
    self.length.partial_cmp(&other.length)
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
    result.push_str(&format!(" · {:>width$}", self.length.thousands(), width = self.dataset.longest_path_display_width));

    // write the string to the formatter
    write!(f, "{}", result)
  }
}

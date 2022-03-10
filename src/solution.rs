use rand::prelude::*;
use std::fmt::Display;
use crate::points_dataset::PointsDataset;
use thousands::Separable;
use std::cmp::{Ordering};

// definition of the solution struct, that represents a possible solutionto the problem
pub struct Solution<'a>  {
  pub points: Vec<usize>,
  pub points_count: usize,
  pub points_dataset: &'a PointsDataset,
  length: Option<f64>,
  pub solution_display_width: usize
}

// implement the Solution struct
impl<'a> Solution<'a> {
  // returns a new random instance of the Solution struct
  pub fn new (points_dataset: &'a PointsDataset, rng: &mut ThreadRng) -> Solution<'a> {
    // generate a list of points as numbers
    let mut points: Vec<usize> = (0..points_dataset.points_count).collect();

    // shuffle the order in which the points are visited
    points.shuffle(rng);

    // create the solution object
    let mut sol = Solution {
      points,
      points_count: points_dataset.points_count,
      points_dataset,
      length: None,
      solution_display_width: points_dataset.points_count * (points_dataset.longest_label_width + 4) - 1 + points_dataset.longest_path_width
    };
    sol.update_length();
    sol
  }

  // // returns an empty instance of the Solution struct
  // pub fn new_empty (points_dataset: &'a PointsDataset) -> Solution<'a> {
  //   Solution { points: vec![], points_dataset, length: None }
  // }

  // // returns an empty instance of the Solution struct generated from a parent solution
  // pub fn new_empty_from_parent (parent_solution: &'a Solution) -> Solution<'a> {
  //   Solution::new_empty(parent_solution.points_dataset)
  // }

  // update the length of the solution
  pub fn update_length(&mut self) {
    // define a variable that will hold the total distance of the solution
    let mut total_length = 0.0;

    // for each point, compute the distance to the next
    for point_index in 0..self.points.len() - 1 {
      total_length += self.points_dataset.points_distances[self.points[point_index]][self.points[point_index + 1]];
    }
    
    // save the length for later
    self.length = Some(total_length);
  }

  // retrieve the length of the solution
  pub fn length(&self) -> f64 {
    match self.length {
      Some(length) => length,
      None => panic!("The length of the solution has not been computed yet")
    }
  }

  // // returns a new instance of the Solution struct generated from two parent solutions
  // pub fn crossover(parent_1: &Solution<'a>, parent_2: &Solution<'a>) -> Solution<'a> {
  //   // create a new empty solution
  //   let mut child = Solution::new_empty_from_parent(parent_1);

  //   // crossover algorithm
  //   for i in 0..child.points_dataset.points_count {
  //     child.points.push(parent_1.points[i]);
  //   }

  //   // return the newly created child
  //   child
  // }

  // mutate randomly the solution
  pub fn mutate (&self, rng: &mut ThreadRng, neighbors_distance_lookup: usize, best_out_of: usize) -> Solution<'a> {
    // we apply the mutation multiple times and only keep the best one
    let mut best_solution: Option<Solution<'a>> = None;

    for _ in 0..best_out_of {
      // let current solution
      let mut child = self.clone();

      // apply inversion mutation
      let mut index_1 = rng.gen_range(0..child.points_count);
      let mut index_2 = rng.gen_range(0..child.points_count);
      if index_1 > index_2 {
        let tmp = index_1;
        index_1 = index_2;
        index_2 = tmp;
      }
      for i in index_1..(index_1 + index_2 + 1)/2 {
        let temp = child.points[i];
        child.points[i] = child.points[index_2 - i + index_1];
        child.points[index_2 - i + index_1] = temp;
      }

      // apply exchange mutation
      let index_1 = rng.gen_range(0..child.points_dataset.points_count);
      let point_1 = child.points[index_1];
      let distance: usize = rng.gen_range(0..neighbors_distance_lookup);
      let point_2 = child.points_dataset.points_nearest_neighbors[point_1][distance];
      let index_2 = child.points.iter().position(|&point| point == point_2).expect("Point not found during mutation exchange");

      child.points[index_1] = point_2;
      child.points[index_2] = point_1;

      // update the length
      child.update_length();

      // save the child if it is the best solution so far
      if best_solution.is_none() || child.length() < best_solution.as_ref().expect("Previous best solution not found").length() {
        best_solution = Some(child);
      }
    }

    best_solution.expect("Best solution not found")
  }
}

  
// implement comparisons operators for the Solution struct
// the comparison is based on the length of the solution
impl<'a> PartialEq for Solution<'a> {
  fn eq (&self, other: &Self) -> bool {
    self.length() == other.length()
  }
}

impl<'a> PartialOrd for Solution<'a> {
  fn partial_cmp (&self, other: &Self) -> Option<Ordering> {
    self.length().partial_cmp(&other.length())
  }
}

// implement the Clone trait for the Solution struct
impl<'a> Clone for Solution<'a> {
  fn clone (&self) -> Self {
    Solution {
      points: self.points.clone(),
      points_count: self.points_count,
      points_dataset: self.points_dataset,
      length: self.length,
      solution_display_width: self.solution_display_width
    }
  }
}

// implement the Display trait for the Solution struct
impl<'sol_lifetime> Display for Solution<'sol_lifetime> {
  fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // create a new string that will contain the stringified solution
    let mut result = String::from("");

    // for each point in the solution, add it to the string
    for (point_index, &point) in self.points.iter().enumerate() {
      result.push_str(&format!("{:>width$}", self.points_dataset.points_labels[point], width = self.points_dataset.longest_label_width));
      if point_index < self.points_dataset.points_count - 1 {
        result.push_str(" -> ");
      }
    }

    // add the length of the solution and a new line
    result.push_str(&format!(" · {:>width$}", self.length().separate_with_commas(), width = self.points_dataset.longest_path_width));

    // write the string to the formatter
    write!(f, "{}", result)
  }
}

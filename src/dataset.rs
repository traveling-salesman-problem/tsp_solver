use std::path::Path;
use std::fs::read_to_string;
use std::fmt::Display;
use std::collections::HashSet;
use serde::Deserialize;
use crate::utils::{get_max_display_width,get_max_display_width_thousands_2d,ThousandsDisplayPolicy};

// alias often reused types
pub type Labels = Vec<String>;
pub type NeighborsMatrix = Vec<Vec<usize>>;

pub enum Matrix {
  Distances(Vec<Vec<f64>>),
  Locations(Vec<(f64, f64)>)
}

impl Matrix {
  pub fn get(&self, node1: usize, node2: usize) -> f64 {
    match self {
      Self::Distances(matrix) => matrix[node1][node2],
      Self::Locations(matrix) => {
        let (p1_x, p1_y) = matrix[node1];
        let (p2_x, p2_y) = matrix[node2];

        let delta_x = p2_x - p1_x;
        let delta_y = p2_y - p1_y;

        (delta_x*delta_x + delta_y*delta_y).sqrt()
      }
    }
  }

  pub fn len(&self) -> usize {
    match self {
      Self::Distances(matrix) => matrix.len(),
      Self::Locations(matrix) => matrix.len()
    }
  }

  pub fn is_square(&self) -> bool {
    match self {
      Self::Distances(matrix) => matrix.len() == matrix[0].len(),
      Self::Locations(matrix) => true
    }
  }

  pub fn max(&self) -> f64 {
    let mut current_max: Option<f64> = None;

    for i in 0..self.len() {
      for j in 0..self.len() {
        match current_max {
          None => { current_max = Some(self.get(i, j)); },
          Some(max) => {
            let d: f64 = self.get(i, j);
            if max < d {
              current_max = Some(d);
            }
          }
        }
      }
    }

    current_max.unwrap()
  }
}

// define a struct to represent a loaded but unverified dataset
// this struct is used to load the dataset from a file using deserialization
#[derive(Deserialize)]
struct UnsafeDataset {
  labels: Labels,
  distance_matrix: Vec<Vec<f64>>,
  locations: Vec<(f64, f64)>
}

// define the structure of the dataset
pub struct Dataset {
  pub size: usize,
  pub labels: Labels,
  pub distance_matrix: Matrix,

  pub longest_path_length: f64,
  pub nodes_neighbors: NeighborsMatrix,

  pub longest_label_display_width: usize,
  pub longest_distance_display_width: usize,
  pub longest_path_display_width: usize,
}

// allow for the creation of a new dataset
impl Dataset {
  // verify that the data is valid
  fn verify(labels: &Labels, distance_matrix: &Matrix) {
    // get number of nodes
    let labels_count = labels.len();

    // there should be at least 2 labels
    if labels_count < 2 {
      panic!("There should be at least 2 nodes in the dataset");
    }

    // verify that labels are unique
    if labels.len() != labels.iter().collect::<HashSet<_>>().len() {
      panic!("Labels should be unique");
    }

    // verify that the number of labels is the same as the number of nodes
    if labels_count != distance_matrix.len() {
      panic!("The number of labels should be the same as the number of nodes : your distances matrix isn't a square");
    }
    if !distance_matrix.is_square() {
      panic!("The number of labels should be the same as the number of nodes : your distances matrix isn't a square");
    }
  }

  // find neighbors for each node
  fn find_neighbors(distance_matrix: &Matrix) -> NeighborsMatrix {
    // create a vector that will contain the neighbors for each node
    let mut node_neighbors: NeighborsMatrix = Vec::new();

    // for each node in the dataset ...
    for node in 0..distance_matrix.len() {
      // create a vector that will contain the nearest neighbors for the current node
      let mut neighbors: Vec<usize> = (0..distance_matrix.len()).collect();

      // sort the nodes by distance to the current node
      neighbors.sort_by(|&n1, &n2| distance_matrix.get(node, n1).partial_cmp(&distance_matrix.get(node, n2)).expect("Error while computing neighbors"));

      // append to the vector of nearest neighbors
      node_neighbors.push(neighbors);
    }

    // return the object
    node_neighbors
  }

  // find the longest possible path length (may actually not be a valid path !)
  fn update_longest_path_length(&mut self) {
    // find the longest value inside the distance matrix
    let mut longest_path_length: f64 = self.distance_matrix.max();

    // multiply by the number of nodes
    longest_path_length *= self.size as f64;

    // update the longest possible path length
    self.longest_path_length = longest_path_length;
    self.longest_path_display_width = longest_path_length.thousands().len();
  }

  // function that allows to create a new dataset object
  pub fn new(labels: Labels, distance_matrix: Matrix) -> Self {
    // verify the dataset
    Self::verify(&labels, &distance_matrix);

    // compute column's widths
    let longest_label_display_width = get_max_display_width(&labels);
    let longest_distance_display_width = get_max_display_width_thousands_2d(&distance_matrix);

    // compute nearest neighbors
    let nodes_neighbors = Self::find_neighbors(&distance_matrix);

    // create and return the object
    let mut dataset = Self {
      size: labels.len(),
      labels,
      distance_matrix,

      longest_path_length: 0.0,
      nodes_neighbors,
      
      longest_label_display_width,
      longest_distance_display_width,
      longest_path_display_width: 0
    };
    dataset.update_longest_path_length();
    dataset
  }

  // function that allows to load a dataset from a file
  pub fn from_file(file_name: &str) -> Self {
    // verify that the given file exists
    if !Path::new(file_name).exists() {
      panic!("The given file does not exist");
    }

    // load the dataset into RAM as a string
    let json_dataset = read_to_string(file_name).expect("Unable to read the dataset file");

    // parse the dataset into an unsafe dataset
    let unsafe_dataset: UnsafeDataset = serde_json::from_str(&json_dataset).expect("Unable to parse the dataset file");

    // create the matrix object
    let matrix = {
      if unsafe_dataset.locations.len() == 0 {
        Matrix::Distances(unsafe_dataset.distance_matrix)
      } else {
        Matrix::Locations(unsafe_dataset.locations)
      }
    };

    // create a new dataset object
    Self::new(unsafe_dataset.labels, matrix)
  }
}

// // implement the Display trait for the Dataset struct
// impl Display for Dataset {
//   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//     // create a string that will contain the dataset in table format
//     let mut string_dataset = String::new();

//     // add padding for the labels columns
//     string_dataset.push_str(&format!("{:>width$}   ", "", width = self.longest_label_display_width));

//     // compute the width of a column
//     let columns_width = usize::max(self.longest_label_display_width, self.longest_distance_display_width);

//     // add each label
//     for (column_index, label) in self.labels.iter().enumerate() {
//       string_dataset.push_str(&format!("{:>width$}", label, width = columns_width));
//       // if the column is not the last one, we add a separator
//       if column_index < self.size - 1 {
//         string_dataset.push_str(" ");
//       }
//     }
//     // add a new line
//     string_dataset.push_str("\n");

//     // add padding for the labels columns
//     string_dataset.push_str(&format!("{:>width$} ┌─", "", width = self.longest_label_display_width));
//     for column_index in 0..self.size {
//       string_dataset.push_str(&format!("{:─>width$}", "", width = columns_width));
//       // if the column is not the last one, we add a separator
//       if column_index < self.size - 1 {
//         string_dataset.push_str("─");
//       }
//     }
//     // add a new line
//     string_dataset.push_str("─┐\n");

//     // for each row in the dataset ...
//     for (row_index, row) in self.distance_matrix.iter().enumerate() {
//       // display the label of the row
//       string_dataset.push_str(&format!("{: >width$} │ ", self.labels[row_index], width = self.longest_label_display_width));
//       // for each value in the row ...
//       for (column_index, &value) in row.iter().enumerate() {
//         // convert the value to underscore if the value is null
//         let value = if value == 0.0 { String::from("_") } else { value.thousands() };
//         // display the value
//         string_dataset.push_str(&format!("{: >width$}", value, width = columns_width));
//         // if not last column, add separator, else add closing
//         if column_index < self.size - 1 {
//           string_dataset.push_str(" ");
//         } else {
//           string_dataset.push_str(" │\n");
//         }
//       }
//     }

//     // display the final row and close the table
//     string_dataset.push_str(&format!("{:>width$} └─", "", width = self.longest_label_display_width));
//     for column_index in 0..self.size {
//       string_dataset.push_str(&format!("{:─>width$}", "", width = columns_width));
//       // if the column is not the last one, we add a separator
//       if column_index < self.size - 1 {
//         string_dataset.push_str("─");
//       }
//     }
//     // add a new line
//     string_dataset.push_str("─┘\n");

//     // write thee string to the formatter
//     write!(f, "{}", string_dataset)
//   }
// }


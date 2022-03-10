// import usefull stuff
use std::path::Path;
use std::fs::read_to_string;
use std::fmt::Display;
use serde::Deserialize;
use thousands::Separable;

// define a struct to represent a loaded but unverified dataset
// this struct is used to load the dataset from a file using deserialization
#[derive(Deserialize)]
struct UnsafePointsDataset {
  points_labels: Vec<String>,
  points_distances: Vec<Vec<f64>>
}

// define the structure of the dataset
pub struct PointsDataset {
  pub points_count: usize,
  pub points_labels: Vec<String>,
  pub points_distances: Vec<Vec<f64>>,

  pub longest_path_length: f64,
  pub points_nearest_neighbors: Vec<Vec<usize>>,

  pub points_columns_widths: Vec<usize>,
  pub longest_label_width: usize,
  pub longest_distance_width: usize,
  pub longest_path_width: usize
}

// allow for the creation of a new dataset
impl PointsDataset {
  // verify that the data is valid
  fn verify_dataset(points_labels: &Vec<String>, points_distances: &Vec<Vec<f64>>) {
    // get number of points
    let labels_count = points_labels.len();

    // there should be at least 2 labels
    if labels_count < 2 {
      panic!("There should be at least 2 points in the dataset");
    }

    // verify that the number of labels is the same as the number of points
    if labels_count != points_distances.len() {
      panic!("The number of labels should be the same as the number of points : your distances matrix isn't a square");
    }
    for row in points_distances.iter() {
      if row.len() != labels_count {
        panic!("The number of labels should be the same as the number of points : your distances matrix isn't a square");
      }
    }
  }

  // compute the width of the longest element in the array
  fn compute_column_width<T: ToString>(column: &Vec<T>) -> usize {
    column.iter().map(|label| label.to_string().len()).max().unwrap()
  }
  
  // compute the widths of all the columns in the array
  fn compute_columns_widths<T: ToString, U: ToString>(columns: &Vec<Vec<T>>, labels: &Vec<U>) -> Vec<usize> {
    // make sure all the columns have the same number of elements
    assert!(columns.iter().map(|row| row.len()).max().unwrap() == columns.iter().map(|row| row.len()).min().unwrap());
    assert!(columns[0].len() == labels.len());
  
    // create an array that will contain the widths of all the columns
    let mut widths: Vec<usize> = Vec::new();
  
    // compute the width of each column
    for column_index in 0..columns[0].len() {
      // get the width of the label of the corresponding column
      let mut column_width: usize = labels[column_index].to_string().len();
  
      // keep widest element
      for row_index in 0..columns.len() {
        let value_width: usize = columns[row_index][column_index].to_string().len();
        if value_width > column_width {
          column_width = value_width;
        }
      }
  
      // append to widths array
      widths.push(column_width);
    }
  
    // return the computed array
    widths
  }

  // find nearest neighbors for each point
  fn generate_nearest_neighbors(points_distances: &Vec<Vec<f64>>) -> Vec<Vec<usize>> {
    // create a vector that will contain the nearest neighbors for each point
    let mut points_nearest_neighbors: Vec<Vec<usize>> = Vec::new();

    // for each point in the dataset ...
    for point in 0..points_distances.len() {
      // create a vector that will contain the nearest neighbors for the current point
      let mut nearest_neighbors: Vec<usize> = (0..points_distances.len()).collect();

      // sort the points by distance to the current point
      nearest_neighbors.sort_by(|&a, &b| points_distances[point][a].partial_cmp(&points_distances[point][b]).expect("Error while computing nearest neighbors"));

      // append to the vector of nearest neighbors
      points_nearest_neighbors.push(nearest_neighbors);
    }

    // return the object
    points_nearest_neighbors
  }

  // find the longest possible path length (may actually not be a valid path !)
  fn update_longest_path_length(&mut self) {
    // copy the distance matrix
    let points_distances_copy = self.points_distances.clone();
    // flatten it to an array of distances
    let mut distances: Vec<f64> = points_distances_copy.into_iter().flatten().collect();
    // sort them by decreasing order
    distances.sort_by(|a, b| b.partial_cmp(a).expect("Error while computing longest possible path"));
    // sum the nth biggest distances
    let mut longest_path_length = 0.0;
    for distance_index in 0..self.points_count {
      longest_path_length += distances[distance_index];
    }
    // update the longest possible path length
    self.longest_path_length = longest_path_length;
    self.longest_path_width = longest_path_length.separate_with_commas().len();
  }

  // function that allows to create a new dataset object
  pub fn new(points_labels: Vec<String>, points_distances: Vec<Vec<f64>>) -> PointsDataset {
    // verify the dataset
    PointsDataset::verify_dataset(&points_labels, &points_distances);

    // compute column's widths
    let longest_label_width = PointsDataset::compute_column_width(&points_labels);
    let points_columns_widths = PointsDataset::compute_columns_widths(&points_distances, &points_labels);

    let longest_distance_width: usize = points_distances.iter().fold(0f64, |total_distance, row| {
      row.iter().fold(total_distance, |sum, &distance| sum + distance)
    }).separate_with_commas().len();

    // compute nearest neighbors
    let points_nearest_neighbors = PointsDataset::generate_nearest_neighbors(&points_distances);

    // create and return the object
    let mut points_dataset = PointsDataset {
      points_count: points_labels.len(),
      points_labels,
      points_distances,

      longest_path_length: 0.0,
      points_nearest_neighbors,
      
      points_columns_widths,
      longest_label_width,
      longest_distance_width,
      longest_path_width: 0
    };
    points_dataset.update_longest_path_length();
    points_dataset
  }

  // function that allows to load a dataset from a file
  pub fn from_file(file_name: &str) -> PointsDataset {
    // verify that the given file exists
    if !Path::new(file_name).exists() {
      panic!("The given file does not exist");
    }

    // load the dataset into RAM as a json string
    let json_dataset = read_to_string(file_name).expect("Unable to read the dataset file");

    // parse the dataset into a RawPointsDataset object
    let raw_dataset: UnsafePointsDataset = serde_json::from_str(&json_dataset).expect("Unable to parse the dataset file");

    // create a PointsDataset object
    PointsDataset::new(raw_dataset.points_labels, raw_dataset.points_distances)
  }
}

// implement the Display trait for the PointsDataset struct
impl Display for PointsDataset {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    // create a string that will contain the dataset in table format
    let mut string_dataset = String::new();

    // add padding for the labels columns
    string_dataset.push_str(&format!("{:>width$}   ", "", width = self.longest_label_width));

    // add each label
    for (column_index, label) in self.points_labels.iter().enumerate() {
      string_dataset.push_str(&format!("{:>width$}", label, width = self.points_columns_widths[column_index]));
      // if the column is not the last one, we add a separator
      if column_index < self.points_count - 1 {
        string_dataset.push_str(" ");
      }
    }
    // add a new line
    string_dataset.push_str("\n");

    // add padding for the labels columns
    string_dataset.push_str(&format!("{:>width$} ┌─", "", width = self.longest_label_width));
    for column_index in 0..self.points_count {
      string_dataset.push_str(&format!("{:─>width$}", "", width = self.points_columns_widths[column_index]));
      // if the column is not the last one, we add a separator
      if column_index < self.points_count - 1 {
        string_dataset.push_str("─");
      }
    }
    // add a new line
    string_dataset.push_str("─┐\n");

    // for each row in the dataset ...
    for (row_index, row) in self.points_distances.iter().enumerate() {
      // display the label of the row
      string_dataset.push_str(&format!("{: >width$} │ ", self.points_labels[row_index], width = self.longest_label_width));
      // for each value in the row ...
      for (column_index, &value) in row.iter().enumerate() {
        // convert the value to underscore if the value is null
        let value = if value == 0.0 { String::from("_") } else { value.to_string() };
        // display the value
        string_dataset.push_str(&format!("{: >width$}", value, width = self.points_columns_widths[column_index]));
        // if not last column, add separator, else add closing
        if column_index < self.points_count - 1 {
          string_dataset.push_str(" ");
        } else {
          string_dataset.push_str(" │\n");
        }
      }
    }

    // display the final row and close the table
    string_dataset.push_str(&format!("{:>width$} └─", "", width = self.longest_label_width));
    for column_index in 0..self.points_count {
      string_dataset.push_str(&format!("{:─>width$}", "", width = self.points_columns_widths[column_index]));
      // if the column is not the last one, we add a separator
      if column_index < self.points_count - 1 {
        string_dataset.push_str("─");
      }
    }
    // add a new line
    string_dataset.push_str("─┘\n");

    // write thee string to the formatter
    write!(f, "{}", string_dataset)
  }
}


use thousands::{SeparatorPolicy,digits,Separable};
use crate::dataset::Matrix;

// function that returns the maximum display width of a vector
pub fn get_max_display_width<T: ToString>(arr: &Vec<T>) -> usize {
  arr.iter().map(|x| x.to_string().len()).max().expect("Unable to find the maximum display width")
}
pub fn get_max_display_width_thousands_2d(matrix: &Matrix) -> usize {
  matrix.max().thousands().len()
}

// implements proper display for big numbers
const THOUSANDS_DISPLAY_POLICY: SeparatorPolicy = SeparatorPolicy {
  separator: "'",
  groups: &[3],
  digits: digits::ASCII_DECIMAL
};

pub trait ThousandsDisplayPolicy {
  fn thousands(&self) -> String;
}

impl<T: Separable> ThousandsDisplayPolicy for T {
  fn thousands(&self) -> String {
    self.separate_by_policy(THOUSANDS_DISPLAY_POLICY)
  }
}

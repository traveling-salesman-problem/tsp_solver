use thousands::{SeparatorPolicy,digits,Separable};

// function that returns the maximum display width of a vector
pub fn get_max_display_width<T: ToString>(arr: &Vec<T>) -> usize {
  arr.iter().map(|x| x.to_string().len()).max().expect("Unable to find the maximum display width")
}
pub fn get_max_display_width_thousands<T: Separable>(arr: &Vec<T>) -> usize {
  arr.iter().map(|x| x.thousands().len()).max().expect("Unable to find the maximum display width")
}

pub fn get_max_display_width_2D<T: ToString>(arr: &Vec<Vec<T>>) -> usize {
  arr.iter().map(|row| get_max_display_width(row)).max().expect("Unable to find the maximum display width")
}
pub fn get_max_display_width_thousands_2D<T: Separable>(arr: &Vec<Vec<T>>) -> usize {
  arr.iter().map(|row| get_max_display_width_thousands(row)).max().expect("Unable to find the maximum display width")
}

// implements proper display for big numbers
const THOUSANDS_DISPLAY_POLICY: SeparatorPolicy = SeparatorPolicy {
  separator: "'",
  groups: &[3, 2],
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

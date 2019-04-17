//returns the missing number, or in the case Vec is empty, returns None
fn missing_number(numbers: &[isize]) -> Option<isize> {
  let mut numbers = Vec::from(numbers);
  numbers.sort();
  let sum: isize = numbers.iter().sum();
  let incremental_sum = ((numbers.len() + 1) as isize * (numbers.last()? + numbers.get(0)?)) / 2;
  Some(incremental_sum - sum)
}


fn main() {
  println!("missing number: {}", missing_number(&[1,3,5,7,11]).unwrap());
}


mod tests {
  use super::missing_number;

  #[test]
  fn returns_none_if_vec_is_empty() {
    assert_eq!(None, missing_number(&[]));
  }

  #[test]
  fn finds_missing_number_sequential_range() {
    assert_eq!(4, missing_number(&[1,2,3,5,6]).unwrap());
  }

  #[test]
  fn finds_missing_number_negative_range() {
    assert_eq!(-5, missing_number(&[-1,-2,-3,-4,-6]).unwrap());
  }

  #[test]
  fn finds_missing_number_evens_range() {
    assert_eq!(10, missing_number(&[2,4,6,8,12]).unwrap());
  }

  #[test]
  fn finds_missing_number_odds_range() {
    assert_eq!(7, missing_number(&[1,3,5,9,11]).unwrap());
  }
}

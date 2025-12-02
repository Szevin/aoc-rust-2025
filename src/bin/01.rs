advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
  input
    .lines()
    .into_iter()
    .map(|line| {
      let (operation, unparsed_amount) = line.split_at(1);

      (operation, unparsed_amount.parse::<i64>().unwrap())
    })
    .fold((50, 0), |(sum, pass), (operation, amount)| {
      let new_sum: i64 = match operation {
        "L" => (sum - amount) % 100,
        "R" => (sum + amount) % 100,
        _ => panic!("Input parse error"),
      };

      let mut new_pass = pass;
      if new_sum == 0 {
        new_pass += 1;
      }

      (new_sum, new_pass)
    })
    .1
    .into()
}

pub fn part_two(input: &str) -> Option<u32> {
  input
    .lines()
    .into_iter()
    .map(|line| {
      let (operation, unparsed_amount) = line.split_at(1);

      (operation, unparsed_amount.parse::<i64>().unwrap())
    })
    .map(|(operation, amount)| match operation {
      "L" => -amount,
      "R" => amount,
      _ => panic!("Input parse error"),
    })
    .fold((50, 0), |(prev_pos, prev_zero_counter), diff| {
      let mut new_pos = prev_pos;
      let mut new_zero_counter = prev_zero_counter;

      let rotations = diff.abs() / 100;
      let relevant_change = diff % 100;

      new_pos += relevant_change;

      if prev_pos != 0 && (new_pos <= 0 || new_pos >= 100) {
        new_zero_counter += 1;
      }

      new_zero_counter += rotations as u32;

      ((100 + new_pos) % 100, new_zero_counter)
    })
    .1
    .into()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(3));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(6));
  }
}

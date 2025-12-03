use itertools::Itertools;
use rayon::{iter::ParallelIterator, str::ParallelString};

advent_of_code::solution!(3);

fn largest_digit_with_index(digits: Vec<u32>) -> (usize, u32) {
  digits
    .iter()
    .enumerate()
    .rev()
    .max_by_key(|pair| *pair.1)
    .map(|(i, &d)| (i, d))
    .unwrap()
}
pub fn part_one(input: &str) -> Option<u32> {
  input
    .par_lines()
    .map(|line| line.chars())
    .map(|chars| {
      chars
        .into_iter()
        .map(|char| char.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
    })
    .map(|digits| {
      let mut first_digit = 0;
      let mut first_digit_pos = 0;
      let mut second_digit = 0;

      let mut i = 0;
      while i < digits.len() - 1 {
        let current_digit = digits[i];

        if current_digit > first_digit {
          first_digit = current_digit;
          first_digit_pos = i;
        }

        i += 1;
      }

      i = first_digit_pos + 1;
      while i < digits.len() {
        let current_digit = digits[i];

        if current_digit > second_digit {
          second_digit = current_digit;
        }

        i += 1;
      }

      first_digit * 10 + second_digit
    })
    // .inspect(|joltage| {dbg!(joltage);})
    .sum::<u32>()
    .into()
}

pub fn part_two(input: &str) -> Option<u128> {
  input
    .par_lines()
    .map(|line| line.chars())
    .map(|chars| {
      chars
        .into_iter()
        .map(|char| char.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
    })
    .map(|digits| {
      let mut res: Vec<(usize, u32)> = vec![];

      for i in 0..12 {
        let diff_to_end = digits.len() - (12 - i) + 1;

        let start = match res.last() {
          Some((index, _)) => index + 1,
          None => 0,
        };

        let current_slice = &digits[start..diff_to_end];

        let mut next_largest = largest_digit_with_index(current_slice.to_vec());

        next_largest.0 += start;
        res.push(next_largest);
      }

      res
    })
    .map(|res| res.iter().map(|(_, digit)| (*digit).to_string()).join(""))
    .map(|str| str.parse::<u128>().unwrap())
    // .inspect(|joltage| {dbg!(joltage);})
    .sum::<u128>()
    .into()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(357));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(3121910778619));
  }
}

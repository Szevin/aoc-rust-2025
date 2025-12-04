use std::u32::MAX;

use ndarray::Array2;
use rayon::{iter::{IntoParallelRefIterator, ParallelIterator}, str::ParallelString};

advent_of_code::solution!(4);

fn increase_neighbours(weight_map: &mut Array2<u32>, x: usize, y: usize) {
  let indicies = vec![
    (-1 as i32, 1 as i32),
    (0, 1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
  ]
  .iter()
  .map(|(x1, y1)| (x1 + x as i32, y1 + y as i32))
  .filter_map(|(x1, y1)| match x1 >= 0 && y1 >= 0 {
    true => Some((x1 as usize, y1 as usize)),
    false => None,
  })
  .collect::<Vec<(usize, usize)>>();

  indicies
    .iter()
    .filter_map(|(x1, y1)| weight_map.get_mut_ptr((*y1, *x1)))
    .for_each(|value| unsafe {
      *value += 1;
    });
}

pub fn part_one(input: &str) -> Option<u32> {
  let lines = input.par_lines().collect::<Vec<&str>>();
  let width = lines.first().unwrap().len();
  let height = lines.len();

  let bit_mask = Array2::<u32>::from_shape_vec(
    (height, width),
    lines
      .par_iter()
      .flat_map_iter(|s| s.chars())
      .map(|c| if c == '@' { 1 } else { 0 })
      .collect(),
  )
  .unwrap();

  let mut weight_map = Array2::<u32>::zeros((width, height));

  for (y, line) in lines.iter().enumerate() {
    for (x, char) in line.chars().enumerate() {
      match char {
        '@' => {
          increase_neighbours(&mut weight_map, x, y);
        }
        _ => {}
      }
    }
  }

  weight_map.zip_mut_with(&bit_mask, |d, &m| match m {
      0 => {
        *d = MAX
      }
      _ => {
        *d *= m
      }
  });

  // dbg!(&weight_map);

  (weight_map
  .iter()
  .map(|&x| x)
  .filter(|&x| x < 4)
  .count() as u32)
  .into()
}

pub fn part_two(input: &str) -> Option<u32> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(13));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}

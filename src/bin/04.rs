use std::usize::MAX;

use ndarray::Array2;
use rayon::{iter::{ParallelIterator}, str::ParallelString};

advent_of_code::solution!(4);

#[derive(Debug)]
struct Coord {
  x: i32,
  y: i32,
}

impl From<(usize, usize)> for Coord {
  fn from((x, y): (usize, usize)) -> Self {
    Coord::new(x as i32, y as i32)
  }
}

impl Clone for Coord {
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone() }
    }
}

impl Coord {
  pub fn new(x: i32, y: i32)-> Self {
    Coord{x, y}
  }

  fn to_usize(&self) -> Option<(usize, usize)> {
    match self.x >= 0 && self.y >= 0 {
        true => Some((self.y as usize, self.x as usize)),
        false => None
    }
  }

  fn generate_neighbour_coords(&self) -> Vec<Coord> {
    vec![
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
  .map(|(x, y)| Coord::new(x + self.x as i32, y + self.y as i32))
  .collect::<Vec<Coord>>()
  }
}

#[derive(Debug)]
enum Tile {
  Empty(Coord),
  Paper(Coord),
}

impl Tile {
    pub fn new(c: char, coord: Coord) -> Self {
      match c {
          '@' => Self::Paper(coord),
          _ => Self::Empty(coord)
      }
    }

    fn get_coord(&self) -> Coord {
      match self {
          Tile::Paper(coord) => coord.clone(),
          Tile::Empty(coord) => coord.clone()
      }
    }

    fn is_empty(&self) -> bool {
      match self {
          Tile::Paper(_) => false,
          Tile::Empty(_) => true
      }
    }
}

#[derive(Debug)]
struct Floor {
  grid: Array2<Tile>
}

impl Floor {
    pub fn new(input: &str)-> Self {
        let lines = input.par_lines().collect::<Vec<&str>>();
        let width = lines.first().unwrap().len();
        let height = lines.len();

        let grid = Array2::<Tile>::from_shape_vec(
          (height, width),
          lines
          .iter().enumerate()
          .map(|(y, line)| {
            line.chars().enumerate().map(|(x, char)| {
              Tile::new(char, Coord::from((x, y)))
            })
            .collect::<Vec<Tile>>()
          })
          .flatten()
          .collect()
        )
        .unwrap();

        Floor { grid }
    }

    fn get_neighbouring_tiles(&self, tile: &Tile) -> Vec<&Tile> {
      let neighbour_coords = tile.get_coord().generate_neighbour_coords();

      neighbour_coords
      .iter()
      .filter_map(|coord| {
        coord.to_usize()
      })
      .filter_map(|coord| {
        self.grid.get(coord)
      })
      .collect::<Vec<&Tile>>()
    }

    fn count_not_empty_neighbours(&self, tile: &Tile) -> usize {
      self
      .get_neighbouring_tiles(tile)
      .iter()
      .filter(|neighbour| !neighbour.is_empty())
      .count()
    }

    fn eliminate(&mut self, remove: bool) -> usize {
      let eliminated_coords: Vec<Coord> = self.grid
        .iter()
        .filter(|&tile| !tile.is_empty())
        .filter(|&tile| self.count_not_empty_neighbours(tile) < 4)
        .map(|tile| tile.get_coord())
        .collect();

      if remove {
        eliminated_coords.iter().for_each(|coord| {
          if let Some((r, c)) = coord.to_usize() {
            if let Some(tile) = self.grid.get_mut((r, c)) {
              *tile = Tile::new(' ', coord.clone());
            }
          }
        })
      }

      eliminated_coords
      .iter()
      .count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
  Floor::new(input).eliminate(false).into()
}

pub fn part_two(input: &str) -> Option<usize> {
  let mut floor = Floor::new(input);

  let mut eliminations: Vec<usize> = vec![];

  while *eliminations.last().unwrap_or(&MAX) != 0 {
    eliminations.push(
      floor.eliminate(true)
    );

    // dbg!(&floor);
    // dbg!(&eliminations);
  };

  eliminations.iter().sum::<usize>().into()
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
    assert_eq!(result, Some(43));
  }
}

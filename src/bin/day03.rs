use std::fs;
use std::collections::HashSet;
use std::convert::TryFrom;

fn main() {
  let contents = fs::read_to_string("src/input/day03.txt")
    .expect("error reading the file");
  let lines = contents.lines();
  let mut trees: HashSet<(u32, u32)> = HashSet::new();
  let mut width: u32 = 0;
  let mut height: u32 = 0;
  for (i, line) in lines.enumerate() {
    width = 0;
    height += 1;
    for (j, c) in line.chars().enumerate() {
      width += 1;
      if c == '#' {
        let x = u32::try_from(j).unwrap();
        let y = u32::try_from(i).unwrap();
        trees.insert((x, y));
      }
    }
  }
  let hill = Hill {
    trees: trees,
    width: width,
    height: height,
  };
  println!("Part One: {}", hill.num_trees(3, 1));
  println!("Part Two: {}", hill.num_trees(1, 1) * hill.num_trees(3, 1) * hill.num_trees(5, 1) * hill.num_trees(7, 1) * hill.num_trees(1, 2));
}

struct Hill {
  trees: HashSet<(u32, u32)>,
  width: u32,
  height: u32,
}

impl Hill {
  fn num_trees(&self, run: u32, rise: u32) -> u32 {
    let mut cur_pos: (u32, u32) = (0, 0);
    let mut count = 0;
    while cur_pos.1 < self.height {
      if self.trees.contains(&cur_pos) { count += 1; }
      let new_x: u32 = (cur_pos.0 + run) % self.width;
      cur_pos = (new_x, cur_pos.1 + rise)
    }
    return count;
  }
}

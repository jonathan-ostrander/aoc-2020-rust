use std::fs;
use std::collections::HashSet;

fn main() {
  let contents = fs::read_to_string("src/input/dayone.txt")
    .expect("error reading the file");
  let lines = contents.lines();
  let mut input = HashSet::new();
  for line in lines {
    input.insert(line.trim().parse::<i64>().expect("unable to parse int"));
  }
  for x in input.iter() {
    if input.contains(&(2020 - x)) {
      println!("Part One: {}", x * (2020 - x));
      break;
    }
  }
  for x in input.iter() {
    for y in input.iter() {
      if input.contains(&(2020 - x - y)) {
        println!("Part Two: {}", x * y * (2020 - x - y));
        break;
      }
    }
  }
}

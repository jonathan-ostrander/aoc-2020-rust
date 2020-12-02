use std::fs;
use std::str::FromStr;
use std::convert::TryFrom;

fn main() {
  let contents = fs::read_to_string("src/input/daytwo.txt")
    .expect("error reading the file");
  let lines = contents.lines();
  let mut valid = 0;
  let mut valid_part_two = 0;
  for line in lines {
    if parse_line(line).is_valid() { valid += 1 ; }
    if parse_line(line).is_valid_by_position() { valid_part_two += 1 ; }
  }
  println!("Part One: {}", valid);
  println!("Part Two: {}", valid_part_two);
}

struct Password<'a> {
  lower_bound: u32,
  upper_bound: u32,
  character: char,
  password: &'a str,
}

impl<'a> Password<'a> {
  fn is_valid(&self) -> bool {
    let mut count = 0;
    for c in self.password.chars() {
      if c == self.character { count += 1; }
    }
    return count >= self.lower_bound && count <= self.upper_bound;
  }

  fn is_valid_by_position(&self) -> bool {
    let f = self.password.chars().nth(usize::try_from(self.lower_bound - 1).unwrap()).unwrap();
    let s = self.password.chars().nth(usize::try_from(self.upper_bound - 1).unwrap()).unwrap();
    (f == self.character) ^ (s == self.character)
  }
}

fn parse_line(s: &str) -> Password {
  let hyphen_split: Vec<&str> = s.splitn(2, "-").collect();
  let lower_bound = hyphen_split.get(0).unwrap();
  let space_split: Vec<&str> = hyphen_split.get(1).unwrap().splitn(2, " ").collect();
  let upper_bound = space_split.get(0).unwrap();
  let colon_split: Vec<&str> = space_split.get(1).unwrap().splitn(2, ": ").collect();
  let character = colon_split.get(0).unwrap().chars().next().unwrap();
  let password = colon_split.get(1).unwrap();
  Password {
    lower_bound: u32::from_str(lower_bound).unwrap(),
    upper_bound: u32::from_str(upper_bound).unwrap(),
    character: character,
    password: password,
  }
}

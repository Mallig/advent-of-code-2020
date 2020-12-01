//use std::env;
use std::fs;

fn main() {
  // Day 1, just use main
    let input = fs::read_to_string("/Users/malachygilchrist/Coding/Projects/Advent-Of-Code/advent_of_code_2020/inputs/day_1.txt")
      .expect("Something went wrong reading the file");
    
    println!("{}", input);
}

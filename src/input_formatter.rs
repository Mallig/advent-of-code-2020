use std::fs;

pub fn day_1() -> Vec<i32> {

  let contents = fs::read_to_string("/Users/malachygilchrist/Coding/Projects/Advent-Of-Code/advent_of_code_2020/src/inputs/day_1.txt").unwrap();

  let mut output: Vec<i32> = Vec::new();

  for i in contents.lines() {
    output.push(i.parse::<i32>().unwrap());
  };

  return output;
}

pub fn day_2() {
  //let contents = fs::read_to_string("/Users/malachygilchrist/Coding/Projects/Advent-Of-Code/advent_of_code_2020/src/inputs/day_2.txt").unwrap();

}

pub fn day_3() {
  //let contents = fs::read_to_string("/Users/malachygilchrist/Coding/Projects/Advent-Of-Code/advent_of_code_2020/src/inputs/day_3.txt").unwrap();

}

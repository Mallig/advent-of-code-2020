use std::fs;

pub fn day_1() -> Vec<i32> {

  let contents = fs::read_to_string("/Users/malachygilchrist/Coding/Projects/Advent-Of-Code/advent_of_code_2020/src/inputs/day_1.txt").unwrap();

  let mut output: Vec<i32> = Vec::new();

  for i in contents.lines() {
    output.push(i.parse::<i32>().unwrap());
  };

  return output;
}

pub fn day_2() -> Vec<Vec<String>> {
  let contents = fs::read_to_string("/Users/malachygilchrist/Coding/Projects/Advent-Of-Code/advent_of_code_2020/src/inputs/day_2.txt").unwrap();

  let p1 = contents.lines();

  let mut output: Vec<Vec<String>> = Vec::new();

  for i in p1 {
    let mut p2: Vec<String> = Vec::new();
    for j in i.split(" ") {
      p2.push(String::from(j));
    }
    output.push(p2)
  }

  return output;
}

pub fn day_3() -> Vec<String> {
  let contents = fs::read_to_string("/Users/malachygilchrist/Coding/Projects/Advent-Of-Code/advent_of_code_2020/src/inputs/day_3.txt").unwrap();

  return read_lines_to_vector(contents);
}


pub fn day_4() -> Vec<String> {
  let contents = fs::read_to_string("/Users/malachygilchrist/Coding/Projects/Advent-Of-Code/advent_of_code_2020/src/inputs/day_4.txt").unwrap();

  let mut output: Vec<String> = Vec::new();

  for i in  contents.split("\n\n") {
    output.push(String::from(i));
  };

  return output;
}

pub fn day_5() -> Vec<String> {
  let contents = fs::read_to_string("/Users/malachygilchrist/Coding/Projects/Advent-Of-Code/advent_of_code_2020/src/inputs/day_5.txt").unwrap();

  return read_lines_to_vector(contents);
}


fn read_lines_to_vector(input: String) -> Vec<String> {
  let mut output : Vec<String> = Vec::new();

  for i in input.lines() {
    output.push(String::from(i));
  };

  return output;
}
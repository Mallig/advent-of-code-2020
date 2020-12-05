
mod input_formatter;
mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
  
  let day_1_input: Vec<i32> = input_formatter::day_1();
  let day_2_input: Vec<Vec<String>> = input_formatter::day_2();
  let day_3_input: Vec<String> = input_formatter::day_3();
  let day_4_input: Vec<String> = input_formatter::day_4();

  print!("Day 1 Solution: ");
  day_1::solve(day_1_input);
  print!("Day 2 Solution: ");
  day_2::solve(day_2_input);
  print!("Day 3 Solution: ");
  day_3::solve(day_3_input);  
  print!("Day 4 Solution: ");
  day_4::solve(day_4_input);

}

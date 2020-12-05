
mod input_formatter;
mod day_1;
mod day_2;
mod day_3;

fn main() {
  
  let day_1_input: Vec<i32> = input_formatter::day_1();
  let day_2_input: Vec<Vec<String>> = input_formatter::day_2();
  let day_3_input: Vec<String> = input_formatter::day_3();

  day_1::solve(day_1_input);
  day_2::solve(day_2_input);
  day_3::solve(day_3_input);

}

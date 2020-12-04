
mod input_formatter;
mod day_1;

fn main() {
  
  let input: Vec<i32> = input_formatter::day_1();

  day_1::solve(input);

}

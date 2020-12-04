mod input_formatter;

pub fn solve() {
  let passwords = input_formatter::day_2();
  let mut valid_count = 0;

  for password in passwords.iter() {
    let mut min_max = Vec::new();

    for s in password[0].split("-") {
      min_max.push(s.parse::<i32>().unwrap())
    }

    // let lettercount = password[2].chars().filter(|c| { *c == password[1].chars().next().unwrap()} ).count() as i32;
    // if lettercount >= min_max[0] && lettercount <= min_max[1] { valid_count += 1 }
    let mut char_count =  0;
    for s in min_max.iter() {
      if password[2].chars().nth((s-1) as usize).unwrap() == password[1].chars().nth(0).unwrap() { char_count += 1; }
    }

    if char_count == 1 { valid_count += 1 }

  }
  println!("{}", valid_count)
}

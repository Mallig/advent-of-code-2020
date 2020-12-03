

// Future optimisations:
//  - on third loop skip odd numbers if current total is even and vice versa
//  - remove max numbers where adding min number would equal or exceed 2020

mod inputs;

pub fn solve() {
  let numbers = inputs.day_1();
  
  let mut minnum = 2020;
  let mut maxnum = 0;

  for number in numbers.iter() {
    minnum = if minnum < *number { minnum } else { *number };
  }

  for number in numbers.iter() {
    maxnum = if maxnum > *number { maxnum } else { *number };
  }

  let mut count1 = 1;
  let mut count2 = 2;

  let mut itercount = 0;

  'outer: for number1 in numbers.iter() {
    if number1 + minnum > 2020 { continue }
    // if number1 + maxnum < 2020 { continue }
    for number2 in numbers[count1..].iter() {
      if number1 + number2 > 2020 { continue }
      if number1 + number2 + minnum > 2020 { continue }
      // if number1 + number2 + maxnum < 2020 { continue }
      for number3 in numbers[count2..].iter() {
        if number1 + number2 + number3 == 2020 {
          println!("{}", number1 * number2 * number3);
          break 'outer;
        }
        count2 += 1;
        itercount += 1;
      }
      count2 = count1 + 1;
    }
    count1 += 1;
  }
  println!("iteration count: {}", itercount);
}

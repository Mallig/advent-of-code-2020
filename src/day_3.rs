
pub  fn solve(trees: Vec<String>) {
  
  let repeat = trees[0].len() -1;

  let slopes = [
    [1, 1],
    [3, 1],
    [5, 1],
    [7, 1],
    [1, 2]
  ];

  let mut x;

  let mut y_mov;
  let mut x_mov;
  let mut hit_count;

  let mut total: i64 = 1;

  for slope in slopes.iter() {
    x_mov = slope[0];
    y_mov = slope[1];
    hit_count = 0;
    x = 0;

    for row in trees.iter().step_by(y_mov) {
      if row.chars().nth(x).unwrap() == "#".chars().nth(0).unwrap() { hit_count += 1 }
      if x + x_mov > repeat { x = x_mov - (repeat - x) - 1 } else { x += x_mov }
    }
    
    total *= hit_count;
  }

  println!("{}", total);
}
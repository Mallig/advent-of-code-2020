pub fn solve(seats: Vec<String>) {
  
  
  let mut seat_list: Vec<Seat> = Vec::new();
  
  for seat in seats.iter() {
    seat_list.push(build_seat(seat.to_string()));
  }
  
  // let mut max_seat_id: i32 = 0;
  // for seat in seat_list {
  //   if seat.id > max_seat_id { max_seat_id = seat.id}
  // }
  // println!("{}", max_seat_id);
  
  let mut plane = build_plane(127, 7);

  for seat in seat_list {
    let index = plane.seat_ids.iter().position(|x| *x == seat.id).unwrap();
    plane.seat_ids.remove(index);
  }

  for seat in plane.seat_ids.iter() {
    if !plane.seat_ids.contains(&(seat-1))
    &&
    !plane.seat_ids.contains(&(seat+1)) {
      println!("{}", seat)
    }
  }
}


pub struct Seat {
  row: i32,
  col: i32,
  id: i32,
  string: String
}

pub struct Plane {
  seat_ids: Vec<i32>
}

fn build_plane(rows: i32, cols: i32) -> Plane {
  let mut seat_ids: Vec<i32> = Vec::new();

  for row in 0..rows+1 {
    for col in 0..cols+1 {
      seat_ids.push((row * 8) + col)
    }
  }

  Plane {
    seat_ids
  }
}

fn build_seat(seat: String) -> Seat {
  let row_num: i32 = get_row_number(seat.clone());
  let col_num: i32 = get_col_number(seat.clone());

  Seat {
    row: row_num,
    col: col_num,
    id: (row_num * 8) + col_num,
    string: seat
  }
}

fn get_row_number(mut seat: String) -> i32 {
  seat.retain(|c| c == 'F' || c == 'B');

  let mut num: i32 = 64;
  let mut row_num: i32 = 0;

  for i in seat.chars() {
    if i == 'B' { row_num += num }
    num /= 2;
  }

  return row_num;
}

fn get_col_number(mut seat: String) -> i32 {
  seat.retain(|c| c == 'R' || c == 'L');

  let mut num: i32 = 4;
  let mut col_num: i32 = 0;

  for i in seat.chars() {
    if i == 'R' { col_num += num }
    num /= 2;
  }

  return col_num;
}

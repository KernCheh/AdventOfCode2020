use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Seat {
  row: i32,
  column: i32,
  pub seat_id: i32,
}

pub fn parse_lines(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> (Vec<Seat>, i32) {
  let mut seats: Vec<Seat> = Vec::new();
  let mut highest_id = 0;

  for line in lines {
    let ll = line.unwrap();
    let seat = parse_into_seat_struct(ll.as_str());
    if seat.seat_id > highest_id {
      highest_id = seat.seat_id;
    }
    seats.push(seat);
  }

  (seats, highest_id)
}

pub fn parse_into_seat_struct(line: &str) -> Seat {
  let chars: Vec<char> = line.chars().collect();

  let mut row_h = 127;
  let mut row_l = 0;
  let mut col_h = 7;
  let mut col_l = 0;

  for c in chars {
    match c {
      'F' => {
        row_h = row_h - ((row_h - row_l + 1) / 2);
      }
      'B' => {
        row_l = row_l + ((row_h - row_l + 1) / 2);
      }
      'L' => {
        col_h = col_h - ((col_h - col_l + 1) / 2);
      }
      'R' => {
        col_l = col_l + ((col_h - col_l + 1) / 2);
      }
      _ => panic!("Unknown char {}", c),
    }
  }

  if row_h != row_l {
    panic!("row_h != row_l, row_h:{} row_l:{}", row_h, row_l);
  }

  if col_h != col_l {
    panic!("col_h != col_l, col_h:{} col_l:{}", col_h, col_l);
  }

  Seat {
    row: row_h,
    column: col_h,
    seat_id: row_h * 8 + col_h,
  }
}

pub fn calculate_missing_seats(seats: &Vec<Seat>) -> Vec<Seat> {
  let mut row_hm: HashMap<i32, HashMap<i32, bool>> = HashMap::new(); // size 128 hash

  for i in 0..128 {
    let mut col_hm: HashMap<i32, bool> = HashMap::new(); // size 8 hash
    for j in 0..8 {
      col_hm.insert(j, false);
    }
    row_hm.insert(i, col_hm);
  }

  for seat in seats {
    row_hm.get_mut(&seat.row).unwrap().insert(seat.column, true);
  }

  let mut unoccupied_seats: Vec<Seat> = Vec::new();

  for (row_id, v) in &row_hm {
    for (col_id, occupied) in v {
      if !*occupied {
        unoccupied_seats.push(Seat {
          row: *row_id,
          column: *col_id,
          seat_id: *row_id * 8 + *col_id,
        });
      }
    }
  }

  unoccupied_seats
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_parses_correctly_1() {
    assert_eq!(
      Seat {
        row: 70,
        column: 7,
        seat_id: 567
      },
      parse_into_seat_struct("BFFFBBFRRR")
    );
  }

  #[test]
  fn it_parses_correctly_2() {
    assert_eq!(
      Seat {
        row: 14,
        column: 7,
        seat_id: 119
      },
      parse_into_seat_struct("FFFBBBFRRR")
    );
  }

  #[test]
  fn it_parses_correctly_3() {
    assert_eq!(
      Seat {
        row: 102,
        column: 4,
        seat_id: 820
      },
      parse_into_seat_struct("BBFFBBFRLL")
    );
  }
}

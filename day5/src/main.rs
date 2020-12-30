use read_input;
mod parse;
use parse::*;

fn main() {
    let lines = read_input::read_input_file_as_lines("input.txt");
    let (seats, highest_id) = parse_lines(lines);
    println!("Highest ID: {}", highest_id);

    let unoccupied_seats = calculate_missing_seats(&seats);

    let missing_seat_ids: Vec<i32> = unoccupied_seats.into_iter().map(|s| s.seat_id).collect();

    for seat_id in &missing_seat_ids {
        if seat_id - 1 < 0 {
            continue;
        }

        if missing_seat_ids.contains(&(*seat_id + 1)) || missing_seat_ids.contains(&(*seat_id - 1))
        {
            continue;
        }

        println!("Missing seat: {}", *seat_id)
    }
}

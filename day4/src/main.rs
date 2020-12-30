mod mylib;
use mylib::read_input;

fn main() {
    let passports = read_input::read_input_file("input.txt");

    let mut valid_passports = 0;
    for passport in &passports {
        if passport.is_valid() {
            valid_passports += 1;
        }
    }

    println!("Part 1: {}", valid_passports);

    valid_passports = 0;

    for passport in &passports {
        if passport.is_valid_2() {
            valid_passports += 1;
        }
    }

    println!("Part 2: {}", valid_passports);
}

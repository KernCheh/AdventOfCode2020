mod read_input;

#[derive(Debug)]
struct Record {
    password_policy: PasswordPolicy,
    password: String,
}

#[derive(Debug)]
struct PasswordPolicy {
    character: char,
    min: i32,
    max: i32,
}

/// Parses the line BufReader, returning a vector of `Record` structure.
///
/// It is not the most performant to parse the entire file into a structure, but we are not
/// too concerned with performance at this point.
fn parse_lines(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> Vec<Record> {
    let mut record_vector = Vec::<Record>::new();
    for line in lines {
        let l = line.unwrap();
        let splitted = l.split(" ").collect::<Vec<&str>>();
        let min_max_str = splitted[0];
        let alphabet = splitted[1].replace(":", "");
        let min_max_vec = min_max_str.split("-").collect::<Vec<&str>>();

        record_vector.push(Record {
            password_policy: PasswordPolicy {
                character: alphabet.chars().collect::<Vec<char>>()[0],
                min: min_max_vec[0].parse::<i32>().unwrap(),
                max: min_max_vec[1].parse::<i32>().unwrap(),
            },
            password: splitted[2].to_string(),
        })
    }

    return record_vector;
}

fn satisfy_policy(password: Vec<char>, character: char, min: i32, max: i32) -> bool {
    let mut occurences = 0;
    for pwd_char in password {
        if pwd_char == character {
            occurences += 1;
        }
    }
    if occurences < min || occurences > max {
        return false;
    }

    true
}

fn satisfy_revised_policy(
    password: Vec<char>,
    character: char,
    lindex: usize,
    rindex: usize,
) -> bool {
    let mut matches = 0;
    if password[lindex - 1] == character {
        matches += 1;
    }

    if password[rindex - 1] == character {
        matches += 1;
    }

    if matches == 1 {
        return true;
    }

    false
}

fn main() {
    let lines = read_input::read_input_file_as_lines("input.txt");
    let records = parse_lines(lines);

    let mut valid = 0;
    for record in &records {
        if satisfy_policy(
            record.password.chars().collect::<Vec<char>>(),
            record.password_policy.character,
            record.password_policy.min,
            record.password_policy.max,
        ) {
            valid += 1;
        }
    }
    println!("Initial policy valid {}", valid);

    valid = 0;
    for record in &records {
        if satisfy_revised_policy(
            record.password.chars().collect::<Vec<char>>(),
            record.password_policy.character,
            record.password_policy.min as usize,
            record.password_policy.max as usize,
        ) {
            valid += 1;
        }
    }

    println!("Revised policy valid {}", valid);
}

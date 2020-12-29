use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f = File::open(&"input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut number_list = Vec::<i32>::new();

    for line in lines {
        let l = line.unwrap();
        let l_int = l.parse::<i32>().unwrap();
        number_list.push(l_int);
    }

    let len = number_list.len();

    let mut ans = 0;
    for i in 0..len - 1 {
        for j in 0..len {
            if i == j {
                continue;
            }

            if number_list[i] + number_list[j] == 2020 {
                ans = number_list[i] * number_list[j];
                break;
            }
        }
    }
    println!("{}", ans);
}

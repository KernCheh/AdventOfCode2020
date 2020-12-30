pub mod passport;
use std::fs::File;
use std::io::{self, BufRead};

/// Read an input file provided by the parameter `filename`.
/// Returns a vector of passports.
pub fn read_input_file(filename: &str) -> Vec<passport::Passport> {
  let mut passport_vec = Vec::<passport::Passport>::new();
  let f = File::open(filename).unwrap();
  let lines = io::BufReader::new(f).lines();

  let mut byr: i32 = 0;
  let mut iyr: i32 = 0;
  let mut eyr: i32 = 0;
  let mut hgt: String = String::new();
  let mut hcl: String = String::new();
  let mut ecl: String = String::new();
  let mut pid: String = String::new();
  let mut cid: i32 = 0;

  for line in lines {
    let ll = line.unwrap();

    if ll != "" {
      let line_vec = ll.split(" ").collect::<Vec<&str>>();

      for item in line_vec {
        let split_item = item.split(":").collect::<Vec<&str>>();
        match split_item[0] {
          "byr" => byr = split_item[1].parse::<i32>().unwrap(),
          "iyr" => iyr = split_item[1].parse::<i32>().unwrap(),
          "eyr" => eyr = split_item[1].parse::<i32>().unwrap(),
          "hgt" => hgt = split_item[1].to_string(),
          "hcl" => hcl = split_item[1].to_string(),
          "ecl" => ecl = split_item[1].to_string(),
          "pid" => pid = split_item[1].to_string(),
          "cid" => cid = split_item[1].parse::<i32>().unwrap(),
          _ => panic!("Unknown key {}", split_item[0]),
        }
      }
    } else {
      let passport = passport::Passport {
        byr: byr,
        iyr: iyr,
        eyr: eyr,
        hgt: hgt,
        hcl: hcl,
        ecl: ecl,
        pid: pid,
        cid: cid,
      };
      passport_vec.push(passport);

      byr = 0;
      iyr = 0;
      eyr = 0;
      hgt = String::new();
      hcl = String::new();
      ecl = String::new();
      pid = String::new();
      cid = 0;
    }
  }

  let passport = passport::Passport {
    byr: byr,
    iyr: iyr,
    eyr: eyr,
    hgt: hgt,
    hcl: hcl,
    ecl: ecl,
    pid: pid,
    cid: cid,
  };
  passport_vec.push(passport);
  passport_vec
}

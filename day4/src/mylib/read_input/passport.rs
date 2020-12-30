#[derive(Debug)]
pub struct Passport {
  pub byr: i32,
  pub iyr: i32,
  pub eyr: i32,
  pub hgt: String,
  pub hcl: String,
  pub ecl: String,
  pub pid: String,
  pub cid: i32,
}

impl Passport {
  pub fn is_valid(&self) -> bool {
    if self.byr == 0 {
      return false;
    }

    if self.iyr == 0 {
      return false;
    }

    if self.eyr == 0 {
      return false;
    }

    if self.hgt == "" {
      return false;
    }

    if self.hcl == "" {
      return false;
    }

    if self.ecl == "" {
      return false;
    }

    if self.pid == "" {
      return false;
    }

    true
  }

  pub fn is_valid_2(&self) -> bool {
    if self.byr < 1920 || self.byr > 2002 {
      return false;
    }

    if self.iyr < 2010 || self.iyr > 2020 {
      return false;
    }

    if self.eyr < 2020 || self.eyr > 2030 {
      return false;
    }

    if self.hgt.ends_with("cm") {
      let hgt_parts = self.hgt.split("cm").collect::<Vec<&str>>();
      let height = hgt_parts[0].parse::<i32>().unwrap();
      if height < 150 || height > 193 {
        return false;
      }
    } else if self.hgt.ends_with("in") {
      let hgt_parts = self.hgt.split("in").collect::<Vec<&str>>();
      let height = hgt_parts[0].parse::<i32>().unwrap();
      if height < 59 || height > 76 {
        return false;
      }
    } else {
      return false;
    }

    if !self.hcl.starts_with("#") {
      return false;
    }

    let hcl_suffix = self.hcl.replace("#", "");
    if hcl_suffix.len() != 6 {
      return false;
    }

    if !hcl_suffix.chars().all(|x| {
      [
        'a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
      ]
      .contains(&x)
    }) {
      return false;
    }

    match self.ecl.as_str() {
      "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => assert!(true),
      _ => {
        return false;
      }
    }

    if !self.pid.chars().all(char::is_numeric) {
      return false;
    }

    if !(self.pid.len() == 9) {
      return false;
    }

    true
  }
}

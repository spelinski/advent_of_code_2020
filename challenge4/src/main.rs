use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Default)]
struct Passport {
    has_byr: bool,
    byr: i32,
    has_iyr: bool,
    iyr: i32,
    has_eyr: bool,
    eyr: i32,
    has_hgt: bool,
    hgt: String,
    has_hcl: bool,
    hcl: String,
    has_ecl: bool,
    ecl: String,
    has_pid: bool,
    pid: String
}

impl Passport {
    fn has_all_fields(&self) -> bool {
        self.has_byr && self.has_iyr && self.has_eyr && self.has_hgt && self.has_hcl && self.has_ecl && self.has_pid
    }

    fn all_fields_valid(&self) -> bool {
        if !self.has_all_fields(){
            return false
        }
        self.byr_valid() && self.iyr_valid() && self.eyr_valid() && self.hgt_valid() && self.hcl_valid() && self.ecl_valid() && self.pid_valid()
    }

    fn pid_valid(&self) -> bool {
        if self.pid.len() != 9 {
            return false;
        }
        !self.pid.parse::<u32>().is_err()
    }
    fn ecl_valid(&self) -> bool {
        match self.ecl.as_ref() {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false
        }
    }
    fn hcl_valid(&self) -> bool {
        if self.hcl.starts_with("#") {
            let sub_string = self.hcl.trim_start_matches("#");
            for character in sub_string.chars() {
                match character {
                    '0'..='9' => {},
                    'a'..='f' => {},
                    _ => {return false}
                }
            }
            if sub_string.len() != 6 { return false }
        } else {
            return false
        }
        true
    }
    fn hgt_valid(&self) -> bool {
        if self.hgt.ends_with("in"){
            let hgt_num :i32 = self.hgt.trim_end_matches("in").parse().unwrap();
            return hgt_num >= 59 && hgt_num <= 76
        } else if self.hgt.ends_with("cm"){
            let hgt_num :i32 = self.hgt.trim_end_matches("cm").parse().unwrap();
            return hgt_num >= 150 && hgt_num <= 193
        }
        false
    }
    fn eyr_valid(&self) -> bool {
        self.eyr >= 2020 && self.eyr <= 2030
    }
    fn iyr_valid(&self) -> bool {
        self.iyr >= 2010 && self.iyr <= 2020
    }
    fn byr_valid(&self) -> bool {
        self.byr >= 1920 && self.byr <= 2002
    }
}

fn read_lines_from_file(filename: &str) -> Result<Vec<String>, Error> {
    let mut return_lines = Vec::new();
    let file_in = File::open(filename)?;
    let all_lines = BufReader::new(file_in).lines();
    for line in all_lines{
        return_lines.push(line.unwrap());
    }
    Ok(return_lines)
}

fn parse_passports(passports: &Vec<String>) -> Vec<Passport> {
    let mut passport_vec = Vec::<Passport>::new();
    let mut temp_passport: Passport = Default::default();
    for line in passports {
        if line.len() == 0 {
            passport_vec.push(temp_passport);
            temp_passport = Default::default();
        }
        let split_line = line.split(" ");
        for key_value in split_line {
            let mut key_value_iter = key_value.split(":");
            let key = key_value_iter.next().unwrap();
            let value = key_value_iter.next().unwrap_or("");
            match key {
                "byr" => {temp_passport.has_byr = true; temp_passport.byr = value.parse().unwrap_or(0);},
                "iyr" => {temp_passport.has_iyr = true; temp_passport.iyr = value.parse().unwrap_or(0);},
                "eyr" => {temp_passport.has_eyr = true; temp_passport.eyr = value.parse().unwrap_or(0);},
                "hgt" => {temp_passport.has_hgt = true; temp_passport.hgt = value.to_string();},
                "hcl" => {temp_passport.has_hcl = true; temp_passport.hcl = value.to_string();},
                "ecl" => {temp_passport.has_ecl = true; temp_passport.ecl = value.to_string();},
                "pid" => {temp_passport.has_pid = true; temp_passport.pid = value.to_string();},
                _ => {}
            }
        }
    }
    passport_vec.push(temp_passport);
    passport_vec
}

fn main() {
    let all_lines = read_lines_from_file("/home/rohk/advent_of_code_2020/challenge4/src/input.txt").expect("Could not load lines"); 

    let all_passports = parse_passports(&all_lines);
    let passports_passed :Vec<Passport> = all_passports.into_iter().filter(|passport| passport.has_all_fields()).collect();
    println!("Number of passports with all fields: {}", passports_passed.len());

    println!("Number of fully valid passports: {}", passports_passed.iter().filter(|&passport| passport.all_fields_valid()).count());
}


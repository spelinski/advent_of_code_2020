use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use regex::Regex;

struct PasswordEntry {
    first_num : usize,
    second_num : usize,
    rule: char,
    password: String
}

fn read_passwords_from_file(filename: &str) -> Result<Vec<PasswordEntry>, Error> {
    let mut all_password_entries = Vec::new();
    let file_in = File::open(filename)?;
    let all_lines = BufReader::new(file_in).lines();
    let re = Regex::new(r"([0-9]+)-([0-9]+)\s+([a-z]+):\s+([a-z]+)").unwrap();
    for line in all_lines{
        let string_of_line = line.expect("Got a bad line from this file");
        let cap = re.captures(string_of_line.as_str()).unwrap();
        all_password_entries.push(PasswordEntry{
            first_num: cap[1].parse().unwrap(),
            second_num: cap[2].parse().unwrap(),
            rule: cap[3].parse().unwrap(),
            password: cap[4].to_string()
        });
    }
    Ok(all_password_entries)
}

fn does_password_meet_rule_sled(password_entry: &PasswordEntry) -> bool {
    let count = password_entry.password.matches(&password_entry.rule.to_string()).count();
    count >= password_entry.first_num && count <= password_entry.second_num
}

fn count_valid_passwords_sled(password_entries: &Vec<PasswordEntry>) -> i32 {
    let mut count = 0;
    for entry in password_entries {
        if does_password_meet_rule_sled(&entry) {
            count += 1;
        }
    }
    count
}

fn does_password_meet_rule_toboggan(password_entry: &PasswordEntry) -> bool {
    let first_idx = password_entry.first_num - 1;
    let last_idx = password_entry.second_num - 1;
    let mut seen = 0;

    if password_entry.password.chars().nth(first_idx).unwrap() == password_entry.rule {
        seen += 1;
    }

    if password_entry.password.chars().nth(last_idx).unwrap() == password_entry.rule {
        seen += 1;
    }

    seen == 1
}

fn count_valid_passwords_toboggan(password_entries: &Vec<PasswordEntry>) -> i32 {
    let mut count = 0;
    for entry in password_entries {
        if does_password_meet_rule_toboggan(&entry) {
            count += 1;
        }
    }
    count
}

fn main() {
    let all_entries = read_passwords_from_file("/home/rohk/advent_of_code_2020/challenge2/src/input.txt").expect("Could not load lines");

    let valid_count = count_valid_passwords_sled(&all_entries);
    println!("Number of valid sled co Passwords: {}", valid_count);

    let valid_count = count_valid_passwords_toboggan(&all_entries);
    println!("Number of valid toboggan co Passwords: {}", valid_count);
}


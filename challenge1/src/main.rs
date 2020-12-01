use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
    iter::FromIterator
};

fn read_nums_to_vec(filename: &str) -> Result<Vec<i32>, Error> {
    let file_in = File::open(filename)?;
    BufReader::new(file_in).lines().map(|line| line.unwrap().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))).collect()
}

fn find_two_that_sum(numbers: &Vec<i32>, target: i32) -> Result<(i32, i32), String> {
    if numbers.len() < 2 {
        return Err("Not enough elements in the vector".to_string());
    }
    let mut first_idx = 0;
    let mut last_idx = numbers.len()-1;
    loop {
        if first_idx == last_idx {
            return Err("Was not able to find two numbers that met requirement".to_string());
        }

        let result = numbers[first_idx] + numbers[last_idx];
        if result == target{
            break;
        }
        if result > target{
            last_idx -= 1;
        } else {
            first_idx += 1;
        }
    }
    Ok((numbers[first_idx], numbers[last_idx]))
}

fn find_three_that_sum(numbers: &Vec<i32>, target: i32) -> Result<(i32, i32, i32), String> {
    let mut last_idx = numbers.len()-1;
    loop {
        if last_idx < 2 {
            break;
        }
        let partial_numbers = Vec::from_iter(numbers[0..last_idx].iter().cloned());
        let result = find_two_that_sum(&partial_numbers, target - numbers[last_idx]);
        match result {
            Ok((first_number, second_number)) => {return Ok((first_number, second_number, numbers[last_idx]))},
            Err(_) => {}
        }
        last_idx -= 1;
    }
    Err("Unable to find a solution with three numbers".to_string())
}

fn main() {
    let mut numbers = read_nums_to_vec("/home/rohk/advent_of_code_2020/challenge1/src/input.txt").expect("Could not load lines");
    numbers.sort();
    //No one after this sort should need to modify the vector
    let sorted_numbers = numbers;

    let result = find_two_that_sum(&sorted_numbers, 2020);
    match result {
        Ok((first_num, second_num)) =>{println!("{}",first_num * second_num);},
        Err(e) => {println!("{}", e);}
    }

    //Just shadow the old result since we don't need it
    let result = find_three_that_sum(&sorted_numbers, 2020);
    match result {
        Ok((first_num, second_num, third_num)) => {println!("{}",first_num*second_num*third_num);},
        Err(e) => {println!("{}", e);}
    }
}


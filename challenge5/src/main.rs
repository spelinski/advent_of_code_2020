use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn read_lines_from_file(filename: &str) -> Result<Vec<String>, Error> {
    let mut return_lines = Vec::new();
    let file_in = File::open(filename)?;
    let all_lines = BufReader::new(file_in).lines();
    for line in all_lines{
        return_lines.push(line.unwrap());
    }
    Ok(return_lines)
}

fn parse_seat(boarding_pass: &String) -> (u32,u32) {
    let mut min_row = 0;
    let mut max_row = 128;
    let mut min_col = 0;
    let mut max_col = 8;
    let mut row = 64;
    let mut col = 4;
    for (i, c) in boarding_pass.chars().enumerate() {
        if i < 6 {
            match c {
                'F' => {max_row = ((max_row - min_row)/2) + min_row;},
                'B' => {min_row = ((max_row - min_row)/2) + min_row;},
                _ => {panic!();}
            }
        } else if i == 6 {
            match c {
                'F' => {row = min_row;},
                'B' => {row = max_row-1;},
                _ => {panic!();}
            }
        } else if i < 9 {
            match c {
                'L' => {max_col = ((max_col - min_col)/2) + min_col;},
                'R' => {min_col = ((max_col - min_col)/2) + min_col;},
                _ => {panic!();}
            }
        } else if i == 9 {
            match c {
                'L' => {col = min_col;},
                'R' => {col = max_col-1;},
                _ => {panic!();}
            }
        }
    }
    (row,col)

}

fn get_seat_ids(seat_row_cols: &Vec<(u32,u32)>) -> Vec<u32> {
    let mut all_seat_ids = Vec::new();
    for (row, col) in seat_row_cols {
        let seat_id = (row * 8) + col;
        all_seat_ids.push(seat_id);
    }
    all_seat_ids
}

fn parse_seats(boarding_passes: &Vec<String>) -> Vec<(u32,u32)> {
    let mut seats = Vec::new();
    for b_pass in boarding_passes {
        seats.push(parse_seat(b_pass));
    }
    seats
}

fn find_my_seat(all_seat_ids: &Vec<u32>) -> u32 {
    let mut sorted_seats = all_seat_ids.clone();
    sorted_seats.sort();
    
    let mut last_seen = 0;
    for seat in sorted_seats {
        //println!("seat: {}", seat);
        if seat == last_seen + 2 {
            return seat - 1
        }
        last_seen = seat.clone();
    }
    last_seen
}

fn main() {
    let all_passes = read_lines_from_file("/home/rohk/advent_of_code_2020/challenge5/src/input.txt").expect("Could not load lines");

    let seats = parse_seats(&all_passes);
    let all_seat_ids = get_seat_ids(&seats);
    println!("Maximum Seat ID: {}", all_seat_ids.iter().max().unwrap());

    let my_seat = find_my_seat(&all_seat_ids);
    println!("My seat id: {}", my_seat);
}


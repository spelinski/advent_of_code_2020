use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn read_slopes_from_file(filename: &str) -> Result<Vec<String>, Error> {
    let mut return_slopes = Vec::new();
    let file_in = File::open(filename)?;
    let all_lines = BufReader::new(file_in).lines();
    for line in all_lines{
        return_slopes.push(line.unwrap());
    }
    Ok(return_slopes)
}

fn count_trees_hit(slopes: &Vec<String>, right: usize, down: usize) -> i64 {
    let mut count_trees = 0;
    let mut current_idx = right;
    let mut first = true;
    let mut skipped = 0;
    let num_to_skip = down - 1;
    for horiz in slopes {
        if first {
            first = false;
            continue;
        }
        if skipped >= num_to_skip {
            let mut horiz_mut = horiz.clone();
            while current_idx >= horiz_mut.len() {
                let temp = horiz.clone();
                horiz_mut.push_str(&temp);
            }
            if horiz_mut.chars().nth(current_idx).unwrap() == '#' {
                count_trees += 1;
            }
            current_idx += right;
            skipped = 0;
        } else {
            skipped += 1;
        }
    }
    count_trees
}

fn main() {
    let all_entries = read_slopes_from_file("/home/rohk/advent_of_code_2020/challenge3/src/input.txt").expect("Could not load lines"); 

    let mut product = 1;

    product *= count_trees_hit(&all_entries, 1, 1);

    let trees_hit = count_trees_hit(&all_entries, 3, 1);
    println!("Answer to part 1: {}", trees_hit);
    product *= trees_hit;

    product *= count_trees_hit(&all_entries, 5, 1);
    product *= count_trees_hit(&all_entries, 7, 1);
    product *= count_trees_hit(&all_entries, 1, 2);

    println!("{}", product);
}


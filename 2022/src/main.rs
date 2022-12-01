mod days;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::path::{Path};

fn main() {
    let input1 = read_lines_to_vector("./inputs/day1.txt").unwrap();
    println!("Answer day 1a {}", days::day1::part1(&input1));
    println!("Answer day 1b {}", days::day1::part2(&input1));
}

fn read_lines_to_vector(file_path: &str) -> Result<Vec<String>, Error>{
    let mut vector = Vec::new();

    let path = Path::new(file_path);
    let file = File::open(path)?;

    
    let filereader = BufReader::new(&file);
    for line in filereader.lines(){
        vector.push(line?)
    }

    Ok(vector)
}
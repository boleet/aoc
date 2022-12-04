mod days;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::path::{Path};

fn main() {
    // let input1 = read_lines_to_vector("./inputs/day1.txt").unwrap();
    // let input2 = read_lines_to_vector("./inputs/day2.txt").unwrap();
    // let input3 = read_lines_to_vector("./inputs/day3.txt").unwrap();
    let input4 = read_lines_to_vector("./inputs/day4.txt").unwrap();


    
    // println!("Answer day 1a {}", days::day1::part1(&input1));
    // println!("Answer day 1b {}", days::day1::part2(&input1));
    // println!("Answer day 2a {}", days::day2::part1(&input2));
    // println!("Answer day 2b {}", days::day2::part2(&input2));
    // println!("Answer day 3a {}", days::day3::part1(&input3));
    // println!("Answer day 3b {}", days::day3::part2(&input3));
      
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
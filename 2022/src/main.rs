mod days;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::path::{Path};

fn main() {
    // let input1 = read_lines_to_vector("./inputs/day1.txt").unwrap();
    // let input2 = read_lines_to_vector("./inputs/day2.txt").unwrap();
    // let input3 = read_lines_to_vector("./inputs/day3.txt").unwrap();
    // let input4 = read_lines_to_vector("./inputs/day4.txt").unwrap();
    // let input5 = read_lines_to_vector("./inputs/day5.txt").unwrap();
    let input6 = read_lines_to_vector("./inputs/day6.txt").unwrap();


    
    // println!("Answer day 1a {}", days::day1::part1(&input1));
    // println!("Answer day 1b {}", days::day1::part2(&input1));
    // println!("Answer day 2a {}", days::day2::part1(&input2));
    // println!("Answer day 2b {}", days::day2::part2(&input2));
    // println!("Answer day 3a {}", days::day3::part1(&input3));
    // println!("Answer day 3b {}", days::day3::part2(&input3));
    // println!("Answer day 4a {}", days::day4::part1(&input4));
    // println!("Answer day 4b {}", days::day4::part2(&input4));
    // println!("Answer day 5a {}", days::day5::part1(&input5));
    // println!("Answer day 5b {}", days::day5::part2(&input5));
    println!("Answer day 6a {}", days::day6::part1(&input6));
    println!("Answer day 6b {}", days::day6::part2(&input6));
      
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
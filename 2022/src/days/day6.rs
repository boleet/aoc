use itertools::Itertools;

#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    for line in input{
        let mut buffer: Vec<char> = Vec::new();
        for c in line.chars(){
            buffer.push(c);
            if buffer.len() >= 4{
                // println!("Buffer window {:?}", buffer.split_at(buffer.len() - 3).1 );
                let frame: Vec<char> = buffer.split_at(buffer.len() - 4).1.to_vec();

                if frame.len() == frame.into_iter().unique().collect_vec().len(){
                    return buffer.len().to_string();
                }
            }
        }
    }

    String::from("Placeholder part 1")
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    for line in input{
        let mut buffer: Vec<char> = Vec::new();
        for c in line.chars(){
            buffer.push(c);
            if buffer.len() >= 14{
                // println!("Buffer window {:?}", buffer.split_at(buffer.len() - 3).1 );
                let frame: Vec<char> = buffer.split_at(buffer.len() - 14).1.to_vec();

                if frame.len() == frame.into_iter().unique().collect_vec().len(){
                    return buffer.len().to_string();
                }
            }
        }
    }


    String::from("Placeholder part 2")
}
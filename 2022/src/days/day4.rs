

#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    let mut sum = 0;
    for line in input{
        let pairs = line.split(",").collect::<Vec<&str>>();
        let (pair_a_start, pair_a_end) = pairs[0].split_once("-").unwrap();
        let (pair_b_start, pair_b_end) = pairs[1].split_once("-").unwrap();
        if pair_a_start.parse::<usize>().unwrap() <= pair_b_start.parse::<usize>().unwrap(){
            if pair_a_end.parse::<usize>().unwrap() >= pair_b_end.parse::<usize>().unwrap(){
                sum += 1;
                continue;
            }
        }
        if pair_a_start.parse::<usize>().unwrap() >= pair_b_start.parse::<usize>().unwrap(){
            if pair_a_end.parse::<usize>().unwrap() <= pair_b_end.parse::<usize>().unwrap(){
                sum += 1;
                continue;
            }
        }
    }
    return sum.to_string();
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    let mut sum = 0;
    for line in input{
        let pairs = line.split(",").collect::<Vec<&str>>();
        let (pair_a_start, pair_a_end) = pairs[0].split_once("-").unwrap();
        let (pair_b_start, pair_b_end) = pairs[1].split_once("-").unwrap();
        // println!("comparing {:?}",pairs );
        if pair_a_end.parse::<usize>().unwrap() <  pair_b_start.parse::<usize>().unwrap() || pair_a_start.parse::<usize>().unwrap() >  pair_b_end.parse::<usize>().unwrap(){

        }else{
            sum += 1;
        }
    }
    return sum.to_string();
}
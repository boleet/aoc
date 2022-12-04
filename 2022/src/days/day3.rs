#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    let mut sum = 0;  
    for line in input{
        let comp1 = &line[..line.len()/2];
        let comp2 = &line[line.len()/2..];
        for char in comp1.chars(){
            if comp2.contains(char) {
                let mut value = (char.to_ascii_lowercase() as u32) - ('a' as u32) +1;
                if char.is_uppercase(){
                    value += 26;
                }
                println!("{:?}", value);
                sum += value; // TODO the value
                break;
            }
        }        
    }
    sum.to_string()
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    let mut sum = 0;  
    let mut group:Vec<&String> = Vec::new();
    
    for line in input{
        group.push(line);
        if group.len() == 3{
            // DO calculation
            for char in group.get(0).unwrap().chars(){
                if group.get(1).unwrap().contains(char) {
                    if group.get(2).unwrap().contains(char) {
                        let mut value = (char.to_ascii_lowercase() as u32) - ('a' as u32) +1;
                        if char.is_uppercase(){
                            value += 26;
                        }
                        sum += value;
                        break;
                    }else{
                        continue;
                    }
                }else{
                    continue;
                }
            }      
            group.clear();
        }
    }
    sum.to_string()
}
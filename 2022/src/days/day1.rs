
#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    let mut max: i64 = 0;
    let mut elf_sum: i64 = 0;

    for line in input{
        if line.is_empty() {
            if elf_sum > max{
                max = elf_sum;
            }
            elf_sum = 0;
        }else{
            elf_sum += line.parse::<i64>().unwrap();
        }
    }

    max.to_string()
}

#[allow(dead_code)]
pub fn part2(input: &Vec<String>) -> String{
    let mut max: Vec<i64> = vec![0,0,0];

    let mut elf_sum: i64 = 0;
    let mut it = input.iter().peekable();
    while let Some(line) = it.next(){
        if line.is_empty() || it.peek().is_none(){
            if it.peek().is_none(){
                elf_sum += line.parse::<i64>().unwrap();
            }
            for i in 0..max.len(){
                if elf_sum > max[i]{
                    max.insert(i, elf_sum);
                    max.pop();
                    break;
                }
            }
            elf_sum = 0;
        }else{
            elf_sum += line.parse::<i64>().unwrap();
        }
    }
    let sum: i64 = max.iter().sum();
    sum.to_string()
}
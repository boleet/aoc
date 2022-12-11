enum COMMAND{
    NOOP,
    ADDX(isize),
}

impl COMMAND {
    fn new(input: &String) -> Self{
        if input.contains("addx"){
            let command: Vec<&str> = input.split(' ').collect();
            let value = command[1].parse().unwrap();
            return COMMAND::ADDX(value)
        }else if input.contains("noop"){
            return COMMAND::NOOP
        }
        COMMAND::NOOP
    }
}


#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    let mut value: isize = 1;
    let mut counter = 0;
    let mut value_history: Vec<isize> = Vec::new();

    for line in input{
        let command = COMMAND::new(line);
        match command{
            COMMAND::NOOP => {
                value_history.push(value);
                counter += 1;
            },
            COMMAND::ADDX(amount) =>{
                value_history.push(value);
                counter += 1;

                value_history.push(value);
                counter+=1;

                value += amount;
            }
        }
        if counter %40 == 0{
            println!();
        }
    }

    let result = value_history.get(19).unwrap() * 20 +
    value_history.get(59).unwrap() * 60 +
    value_history.get(99).unwrap() * 100 +
    value_history.get(139).unwrap() * 140 +
    value_history.get(179).unwrap() * 180 +
    value_history.get(219).unwrap() * 220;

    result.to_string()
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    let mut value: isize = 1;
    let mut counter = 0;
    let mut value_history: Vec<isize> = Vec::new();

    for line in input{
        let command = COMMAND::new(line);
        match command{
            COMMAND::NOOP => {
                value_history.push(value);
                print_pixel(counter, value);
                counter += 1;
            },
            COMMAND::ADDX(amount) =>{
                value_history.push(value);
                print_pixel(counter, value);
                counter += 1;

                

                value_history.push(value);
                print_pixel(counter, value);
                counter+=1;

                value += amount;
            }
        }
        
    }
    String::from("Answer has to be read from terminal")
}

fn print_pixel(counter: isize, value: isize){
    if counter %40 == 0{
        println!();
    }

    if counter%40 >= value -1 && counter %40 <= value + 1{
        print!("#");
    }else{
        print!(".");
    }
}
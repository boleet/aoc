#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    let mut stacks: Vec<Vec<String>> = Vec::new();

    let mut start_setup_initialized = false;

    for line in input{
        if !start_setup_initialized{
            // Read start state
            if line.contains('1'){
                start_setup_initialized = true;
                continue;
            }

            // let mut line_chars = line.chars().collect();
            let stacks_amount = (line.len() as f32 / 4.0).ceil() as usize;
            for i in 0..stacks_amount{
                if stacks.len() <= i{
                    stacks.push(Vec::new());
                }
                match line.chars().nth(i*4 + 1){
                    Some(' ') => {},
                    Some(v) => stacks.get_mut(i).unwrap().insert(0, v.to_string()),
                    _ =>{}
                };
                
            }
        }else if line.is_empty(){
            continue;
        }else{
            // Move commands
            let params: Vec<usize> = line.split(" ")
            .filter(|x| x.parse::<usize>().is_ok())
            .map(|x| x.parse().unwrap())
            .collect();
            for _ in 0..params[0]{
                // println!("Executing move from {} to {}", params[1], params[2]);
                let elem = stacks.get_mut(params[1]-1).unwrap().pop().unwrap();
                let target_stack = stacks.get_mut(params[2]-1).unwrap();
                target_stack.push(elem);
                // println!("Now stack looks like {:?}", stacks);
            }
            
        }
    }

    let mut result = String::new();
    for stack in stacks{
        result += stack.last().unwrap();
    }
    result
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    let mut stacks: Vec<Vec<String>> = Vec::new();

    let mut start_setup_initialized = false;

    for line in input{
        if !start_setup_initialized{
            // Read start state
            if line.contains('1'){
                start_setup_initialized = true;
                continue;
            }

            // let mut line_chars = line.chars().collect();
            let stacks_amount = (line.len() as f32 / 4.0).ceil() as usize;
            for i in 0..stacks_amount{
                if stacks.len() <= i{
                    stacks.push(Vec::new());
                }
                match line.chars().nth(i*4 + 1){
                    Some(' ') => {},
                    Some(v) => stacks.get_mut(i).unwrap().insert(0, v.to_string()),
                    _ =>{}
                };
                
            }
        }else if line.is_empty(){
            continue;
        }else{
            // Move commands
            let params: Vec<usize> = line.split(" ")
            .filter(|x| x.parse::<usize>().is_ok())
            .map(|x| x.parse().unwrap())
            .collect();
            let mut elems = Vec::new();
            for j in 0..params[0]{
                // println!("Executing move from {} to {}", params[1], params[2]);
                elems.push(stacks.get_mut(params[1]-1).unwrap().pop().unwrap());
                // println!("Now stack looks like {:?}", stacks);
            }
            let target_stack = stacks.get_mut(params[2]-1).unwrap();
            elems.reverse();
            target_stack.append(&mut elems);
            
        }
    }

    let mut result = String::new();
    for stack in stacks{
        result += stack.last().unwrap();
    }
    result
}
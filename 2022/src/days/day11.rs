use std::vec;

#[derive(Clone, Copy, Debug)]
enum Operation{
    ADD(OperationPrimitive),
    MULT(OperationPrimitive),
    NOP
}

#[derive(Clone, Copy, Debug)]
enum OperationPrimitive{
    SELF,
    NUM(usize),
}

impl Operation{
    fn new(inp: &str) -> Self{
        if inp.contains("+"){
            let value_str = inp.split("+").last().unwrap();
            let value: OperationPrimitive;
            if value_str.contains("old"){
                value = OperationPrimitive::SELF;
            }else{
                value = OperationPrimitive::NUM(value_str.trim().parse::<usize>().unwrap());
            }
            Operation::ADD(value)
        }else if inp.contains("*"){
            let value_str = inp.split("*").last().unwrap();
            let value: OperationPrimitive;
            if value_str.contains("old"){
                value = OperationPrimitive::SELF;
            }else{
                value = OperationPrimitive::NUM(value_str.trim().parse::<usize>().unwrap());
            }
            Operation::MULT(value)
        }else{
            Operation::NOP
        }
    }
}

#[derive(Debug)]
struct Monkey{
    items: Vec<usize>,
    operation: Operation,
    divisble_by: usize,
    true_throw: usize,
    false_throw: usize,
    inspects: usize,
}

impl Monkey{
    fn process_items(&mut self, divide_three: bool) -> Vec<(usize, usize)>{
        let mut result = vec![]; // (new_id, new_value)

        for item in &mut self.items{
            let mut new_item: usize = *item;
            match &self.operation {
                Operation::ADD(v) =>{
                    match v{
                        OperationPrimitive::SELF => {
                            new_item += new_item;
                        },
                        OperationPrimitive::NUM(x) =>{
                            new_item += x;
                        }                    
                    }
                },
                Operation::MULT(v) =>{
                    match v{
                        OperationPrimitive::SELF => {
                            new_item *= new_item;
                        },
                        OperationPrimitive::NUM(x) =>{
                            new_item *= x;
                        } 
                    }    
                },
                Operation::NOP =>{
                    unimplemented!()
                }
            }
            if divide_three{
            new_item = new_item /3;
            }else{
                new_item = new_item % 9699690; // TODO manually calculated from the input value, not nice
            }


            if new_item % self.divisble_by == 0{
                result.push((self.true_throw, new_item));
            }else{
                result.push((self.false_throw, new_item));
            }
            self.inspects += 1;
        }
        self.items.clear();

        result
    }

    fn add_item(&mut self, value: usize){
        self.items.push(value);
    }
}



#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut m_id = 0;
    let mut m_items: Vec<usize> = vec![];
    let mut m_operation = Operation::NOP;
    let mut m_divisible_by = 0;
    let mut m_true_throw = 0;
    let mut m_false_throw = 0;
    
    for line in input{
        if line.contains("Monkey"){
            m_id = line.replace("Monkey ", "").replace(":", "").parse().unwrap();
        }else if line.contains("Starting items"){
            let cmd: Vec<&str> = line.split(":").collect();
            m_items =  cmd.get(1).unwrap().trim().split(",").map(|x| x.trim().parse::<usize>().unwrap()).collect();
        }else if line.contains("Operation"){  
            let cmd = line.split("=").last().unwrap();
            m_operation = Operation::new(cmd);
        }else if line.contains("Test"){
            m_divisible_by= line.split(" ").last().unwrap().parse::<usize>().unwrap();
        }else if line.contains("If true"){
            m_true_throw = line.split(" ").last().unwrap().parse::<usize>().unwrap();
        }else if line.contains("If false"){
            m_false_throw = line.split(" ").last().unwrap().parse::<usize>().unwrap();
        }else if line.is_empty(){
            let new_monkey = Monkey { 
                items: m_items.to_vec(), 
                operation: m_operation, 
                divisble_by: m_divisible_by, 
                true_throw: m_true_throw, 
                false_throw: m_false_throw, 
                inspects: 0,
            };
            monkeys.insert(m_id,new_monkey);
        }
    }
    // file does not end with new line..
    let new_monkey = Monkey { 
        items: m_items.to_vec(), 
        operation: m_operation, 
        divisble_by: m_divisible_by, 
        true_throw: m_true_throw, 
        false_throw: m_false_throw, 
        inspects:0,
    };
    monkeys.insert(m_id,new_monkey);


    for _ in 0..20{
        // calculating rounds
        for monkey_i in 0..monkeys.len(){
            let changed_items = monkeys.get_mut(monkey_i).unwrap().process_items(true);
            for item in changed_items{
                // move item.1 to monkey item.0
                monkeys.get_mut(item.0).unwrap().add_item(item.1);
            }
        }
    }


    let mut monkey_inspects_top: Vec<usize> = vec![];
    for monkey_i in 0..monkeys.len(){
        let monkey = monkeys.get(monkey_i).unwrap();
        if monkey_inspects_top.len() < 2{
            if monkey_inspects_top.len() > 0 && monkey.inspects > monkey_inspects_top[0]{
                monkey_inspects_top.insert(0, monkey.inspects);
            }else{
                monkey_inspects_top.push(monkey.inspects);
            }
        }else{
            if monkey.inspects > monkey_inspects_top[0]{
                monkey_inspects_top.insert(0, monkey.inspects);
                monkey_inspects_top.pop();
            }else if monkey.inspects > monkey_inspects_top[1]{
                monkey_inspects_top.insert(1, monkey.inspects);
                monkey_inspects_top.pop();
            }
        }
    }
    (monkey_inspects_top[0] * monkey_inspects_top[1]).to_string()
}


// TODO copy past part1 is very ugly
// TODO for part 2 we need to do modulo to make it work, not integer overflow
#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut m_id = 0;
    let mut m_items: Vec<usize> = vec![];
    let mut m_operation = Operation::NOP;
    let mut m_divisible_by = 0;
    let mut m_true_throw = 0;
    let mut m_false_throw = 0;
    
    for line in input{
        if line.contains("Monkey"){
            m_id = line.replace("Monkey ", "").replace(":", "").parse().unwrap();
        }else if line.contains("Starting items"){
            let cmd: Vec<&str> = line.split(":").collect();
            m_items =  cmd.get(1).unwrap().trim().split(",").map(|x| x.trim().parse::<usize>().unwrap()).collect();
        }else if line.contains("Operation"){  
            let cmd = line.split("=").last().unwrap();
            m_operation = Operation::new(cmd);
        }else if line.contains("Test"){
            m_divisible_by= line.split(" ").last().unwrap().parse::<usize>().unwrap();
        }else if line.contains("If true"){
            m_true_throw = line.split(" ").last().unwrap().parse::<usize>().unwrap();
        }else if line.contains("If false"){
            m_false_throw = line.split(" ").last().unwrap().parse::<usize>().unwrap();
        }else if line.is_empty(){
            let new_monkey = Monkey { 
                items: m_items.to_vec(), 
                operation: m_operation, 
                divisble_by: m_divisible_by, 
                true_throw: m_true_throw, 
                false_throw: m_false_throw, 
                inspects: 0,
            };
            monkeys.insert(m_id,new_monkey);
        }
    }
    // file does not end with new line..
    let new_monkey = Monkey { 
        items: m_items.to_vec(), 
        operation: m_operation, 
        divisble_by: m_divisible_by, 
        true_throw: m_true_throw, 
        false_throw: m_false_throw, 
        inspects:0,
    };
    monkeys.insert(m_id,new_monkey);


    for _ in 0..10000{
        // calculating rounds
        for monkey_i in 0..monkeys.len(){
            let changed_items = monkeys.get_mut(monkey_i).unwrap().process_items(false);
            for item in changed_items{
                // move item.1 to monkey item.0
                monkeys.get_mut(item.0).unwrap().add_item(item.1);
            }
        }
    }


    let mut monkey_inspects_top: Vec<usize> = vec![];
    for monkey_i in 0..monkeys.len(){
        let monkey = monkeys.get(monkey_i).unwrap();
        if monkey_inspects_top.len() < 2{
            if monkey_inspects_top.len() > 0 && monkey.inspects > monkey_inspects_top[0]{
                monkey_inspects_top.insert(0, monkey.inspects);
            }else{
                monkey_inspects_top.push(monkey.inspects);
            }
        }else{
            if monkey.inspects > monkey_inspects_top[0]{
                monkey_inspects_top.insert(0, monkey.inspects);
                monkey_inspects_top.pop();
            }else if monkey.inspects > monkey_inspects_top[1]{
                monkey_inspects_top.insert(1, monkey.inspects);
                monkey_inspects_top.pop();
            }
        }
    }
    (monkey_inspects_top[0] * monkey_inspects_top[1]).to_string()
}
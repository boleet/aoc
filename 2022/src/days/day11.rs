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
    id: usize,
    items: Vec<usize>,
    operation: Operation,
    divisble_by: usize,
    true_throw: usize,
    false_throw: usize,
    inspects: usize,
}

impl Monkey{
    fn process_items(&mut self) -> Vec<(usize, usize)>{
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
            new_item = new_item /3; // TODO ceil instead of integer division
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
                id: m_id, 
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
        id: m_id, 
        items: m_items.to_vec(), 
        operation: m_operation, 
        divisble_by: m_divisible_by, 
        true_throw: m_true_throw, 
        false_throw: m_false_throw, 
        inspects:0,
    };
    monkeys.insert(m_id,new_monkey);


    for round_i in 0..20{
        // calculating rounds
        for monkey_i in 0..monkeys.len(){
            let changed_items = monkeys.get_mut(monkey_i).unwrap().process_items();
            for item in changed_items{
                // move item.1 to monkey item.0
                monkeys.get_mut(item.0).unwrap().add_item(item.1);
            }
        }
    }


    let monkey_inspects: Vec<usize> = vec![0,0];
    for monkey_i in 0..monkeys.len(){
        let monkey = monkeys.get(monkey_i).unwrap();
        println!("Monkey {} with {} inspects has items {:?}", monkey.id, monkey.inspects, monkey.items);
        // if monkey.inspects > monkey_inspects.get(0)
    }
    println!();


    

    String::from("Placeholder part 1")
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    for line in input{
       
    }

    String::from("Placeholder part 2")
}
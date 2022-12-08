// use core::panic;
// use std::cell::RefCell;
// use std::rc::Rc;

// // https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
// struct Node {
//     pub name: String,
//     pub value: usize,
//     pub children: Vec<Rc<RefCell<Node>>>,
//     pub parent: Option<Rc<RefCell<Node>>>
// }

// impl Node{
//     fn add_child(&mut self, n: Rc<RefCell<Node>>){
//         self.children.push(n);
//     }
// }

// #[allow(dead_code)]
// pub fn part1(input: &Vec<String>) -> String{
//     let mut tree = Rc::new(RefCell::new(Node {name: "/".to_string(), value: 0, children: vec![], parent: None }));
//     let mut current = Rc::clone(&tree);

//     let mut sum = 0;
        
//     for line in input{
//         let command: Vec<&str> = line.split(' ').collect();

//         match command[0]{
//             "$" => {
//                 match command[1]{
//                     "cd" =>{
//                         println!("cd command {:?}", command);
//                         if command[2].eq("/"){
//                             continue; // skip for now
//                         }
//                         if command[2].eq(".."){
//                             let b = Rc::clone(current.borrow().parent.unwrap());
//                             current = b;
//                         }else{
//                             let b = Rc::clone(current.borrow().children.first().unwrap());
//                             current = b;
//                         }
                        
//                     },
//                     "ls" =>{
//                         println!("ls command {:?}", command);
//                     },
//                     _ => !panic!("Whoops")
//                 }
//             },
//             "dir" => {
//                 println!("Directory found! {:?}", command);
//                 let new_node = Rc::new(RefCell::new(Node {name: command[1].to_string(), value: 0, children: vec![], parent: None }));
//                 current.borrow_mut().add_child(Rc::clone(&new_node));
//             },
//             _ => {
//                 println!("File found! {:?}", command);
//                 let new_node = Rc::new(RefCell::new(Node {name: command[1].to_string(), value: command[0].parse().unwrap(), children: vec![], parent: None }));
//                 current.borrow_mut().add_child(Rc::clone(&new_node));
//             }
//         }
//     }

//     String::from("Placeholder part 1")
// }


// #[allow(dead_code, unused_variables)]
// pub fn part2(input: &Vec<String>) -> String{
//     for line in input{
       
//     }

//     String::from("Placeholder part 2")
// }
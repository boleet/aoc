use core::panic;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::default;
use std::hash::Hash;
use std::rc::Rc;

//https://fasterthanli.me/series/advent-of-code-2022/part-7#using-rc-refcell
#[derive(Default)]
struct Node{
    name: String,
    value: usize,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>
}

impl Node{
    fn new(name: String, value: usize) -> Self{
        Node { 
            name: name, 
            value: value, 
            children: Vec::new(),
            parent: None, 
        }
    }
}

#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    let root = Rc::new(RefCell::new(Node::new("/".to_string(), 0)));
    let mut node: Rc<RefCell<Node>> = root.clone();

    let mut sum = 0;
        
    for line in input{
        let command: Vec<&str> = line.split(' ').collect();

        match command[0]{
            "$" => {
                match command[1]{
                    "cd" =>{
                        println!("cd command {:?}", command);
                        if command[2].eq("/"){
                            continue; // skip for now
                        }
                        if command[2].eq(".."){
                            // TODO current <- parent
                            
                        }else{
                            // TODO current <- correct child
                           
                        }
                        
                    },
                    "ls" =>{
                        println!("ls command {:?}", command);
                    },
                    _ => !panic!("Whoops")
                }
            },
            "dir" => {
                println!("Directory found! {:?}", command);
                // TODO add node to tree
            },
            _ => {
                println!("File found! {:?}", command);
                // TODO add node with value here 
            }  
        }
    }

    String::from("Placeholder part 1")
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    for line in input{
       
    }

    String::from("Placeholder part 2")
}
use core::panic;
use std::cell::RefCell;
use std::rc::Rc;

//https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
#[derive(Debug)]
struct Node{
    name: String,
    value: usize,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
    directory: bool
}

impl Node{
    fn new(name: String, value: usize, parent: Option<Rc<RefCell<Node>>>, directory: bool) -> Self{
        Node { 
            name: name, 
            value: value, 
            children: Vec::new(),
            parent: parent, 
            directory: directory,
        }
    }

    fn add_child(&mut self, new_node: Rc<RefCell<Node>>){
        self.children.push(new_node);
    }

    fn set_size(&mut self) -> usize{
        if self.value != 0{
            return self.value;
        }else{
            let mut mysize: usize = 0;
            for child in &mut self.children{
                mysize += child.borrow_mut().set_size();
            }
            self.value = mysize;
            return mysize;
        }
    }

    fn sum_small_directories(&self) -> usize{
        let mut res: usize = 0;
        if self.directory && self.value < 100000{
            res += self.value;
        }
        for child in &self.children{
            res += child.borrow_mut().sum_small_directories();
        }
        return res;
    }

    fn find_directory_sized_closest_to(&self, target: usize) -> usize{
        let mut closest: usize = 0;
        if self.directory && self.value >= target{
            closest = self.value;
        }
        for child in &self.children{
            let c = child.borrow_mut().find_directory_sized_closest_to(target);
            if c >= target{
                if closest == 0 || (closest - target) > (c - target) {
                    closest = c;
                }
            }
        }
        return closest;
    }

}

#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String{
    let root = create_file_tree(input);
    let current_loop: Rc<RefCell<Node>> = Rc::clone(&root);
    current_loop.borrow_mut().set_size();

    let x =current_loop.borrow().sum_small_directories();
    x.to_string()
}


#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String{
    let root = create_file_tree(input);
    let current_loop: Rc<RefCell<Node>> = Rc::clone(&root);
    current_loop.borrow_mut().set_size();

    let space_used = current_loop.borrow().value;
    let space_to_free = 30000000 - (70000000 - space_used);
    println!("Trying to free {}", space_to_free);
    let x = current_loop.borrow().find_directory_sized_closest_to(space_to_free);
    x.to_string()
}

fn create_file_tree(input: &Vec<String>) -> Rc<RefCell<Node>>{
    let root = Rc::new(RefCell::new(Node::new("/".to_string(), 0, None, true)));
    let mut current: Rc<RefCell<Node>> = Rc::clone(&root);

    let mut sum = 0;
        
    for line in input{
        let command: Vec<&str> = line.split(' ').collect();

        match command[0]{
            "$" => {
                match command[1]{
                    "cd" =>{
                        // println!("cd command {:?}", command);
                        if command[2].eq("/"){
                            continue; // skip for now
                        }
                        if command[2].eq(".."){
                            // TODO current <- parent
                            let current_clone: Rc<RefCell<Node>> = Rc::clone(&current);
                            current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                        }else{
                            // TODO current <- correct child
                            let current_clone = Rc::clone(&current);
                            current = Rc::clone(current_clone.borrow().children.iter().find(|x| x.borrow().name == command[2]).unwrap());
                        }
                        
                    },
                    "ls" =>{
                        // println!("ls command {:?}", command);
                    },
                    _ => !panic!("Whoops")
                }
            },
            "dir" => {
                // println!("Directory found! {:?}", command);
                // TODO add node to tree
                let new_child = Rc::new(RefCell::new(Node::new(command[1].to_string(), 0, Some(Rc::clone(&current)), true)));
                current.borrow_mut().add_child(Rc::clone(&new_child));
            },
            _ => {
                // println!("File found! {:?}", command);
                // TODO add node with value here 
                let new_child = Rc::new(RefCell::new(Node::new(command[1].to_string(), command[0].parse().unwrap(), Some(Rc::clone(&current)), false)));
                current.borrow_mut().add_child(Rc::clone(&new_child));
            }  
        }
    }
    return root;
}


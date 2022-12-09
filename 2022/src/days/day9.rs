use std::collections::HashSet;

struct Rope {
    head: (isize, isize), // x,y bottom left = 0,0
    // tail: (isize, isize),
    tail: Vec<(isize, isize)>,
    size: usize, // size, besides the head
}

impl Rope {
    fn new(size: usize) -> Self {
        Rope {
            head: (0, 0),
            tail: vec![(0,0); size],
            size: size, 
        }
    }

    fn move_left(&mut self) {
        self.head.0 -= 1;
        self.update_tail();
    }
    fn move_right(&mut self) {
        self.head.0 += 1;
        self.update_tail();
    }
    fn move_up(&mut self) {
        self.head.1 += 1;
        self.update_tail();
    }
    fn move_down(&mut self) {
        self.head.1 -= 1;
        self.update_tail();
    }

    fn update_tail(&mut self) {
        for i in 0..self.size{
            let mut hx: isize = 0;
            let mut hy: isize = 0;
            if i == 0{
                hx = self.head.0;
                hy = self.head.1;
            }else{
                hx = self.tail.get(i-1).unwrap().0;
                hy = self.tail.get(i-1).unwrap().1;
            }
            let tx: isize = self.tail.get(i).unwrap().0;
            let ty: isize = self.tail.get(i).unwrap().1;
            if (hx - tx).abs() > 1 || (hy - ty).abs() > 1 {
                // we need to update the tail
                if hx == tx {
                    // only need to move y
                    if hy < ty {
                        self.tail.get_mut(i).unwrap().1 -= 1;
                    } else {
                        self.tail.get_mut(i).unwrap().1 += 1;
                    }
                } else if hy == ty {
                    // only need to move x
                    if hx < tx {
                        self.tail.get_mut(i).unwrap().0 -= 1;
                    } else {
                        self.tail.get_mut(i).unwrap().0 += 1;
                    }
                } else {
                    // need to move diagonally
                    if hx > tx{
                        self.tail.get_mut(i).unwrap().0 += 1
                    }else{
                        self.tail.get_mut(i).unwrap().0 -= 1;
                    }

                    if hy > ty{
                        self.tail.get_mut(i).unwrap().1 += 1
                    }else{
                        self.tail.get_mut(i).unwrap().1 -= 1;
                    }
                }
            }else{
                break;
            }
        }
    }

    fn get_tail(&self) -> (isize, isize) {
        *self.tail.last().unwrap()
    }
}

#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String {
    let mut rope = Rope::new(1);
    let mut unique_locations: HashSet<(isize, isize)> = HashSet::new();

    unique_locations.insert(rope.get_tail());

    for line in input {
        let command: Vec<&str> = line.split(' ').collect();
        let amount: usize = command[1].parse().unwrap();
        match command[0] {
            "R" => {
                for _ in 0..amount {
                    rope.move_right();
                    unique_locations.insert(rope.get_tail());
                }
            }
            "L" => {
                for _ in 0..amount {
                    rope.move_left();
                    unique_locations.insert(rope.get_tail());
                }
            }
            "U" => {
                for _ in 0..amount {
                    rope.move_up();
                    unique_locations.insert(rope.get_tail());
                }
            }
            "D" => {
                for _ in 0..amount {
                    rope.move_down();
                    unique_locations.insert(rope.get_tail());
                }
            }
            _ => {}
        }
    }
    // println!("Positions: {:?}", unique_locations);
    unique_locations.len().to_string()
}

#[allow(dead_code, unused_variables)]
pub fn part2(input: &Vec<String>) -> String {
    println!("Start part 2");
    let mut rope = Rope::new(9);
    let mut unique_locations: HashSet<(isize, isize)> = HashSet::new();

    unique_locations.insert(rope.get_tail());

    for line in input {
        let command: Vec<&str> = line.split(' ').collect();
        let amount: usize = command[1].parse().unwrap();
        match command[0] {
            "R" => {
                for _ in 0..amount {
                    rope.move_right();
                    unique_locations.insert(rope.get_tail());
                }
            }
            "L" => {
                for _ in 0..amount {
                    rope.move_left();
                    unique_locations.insert(rope.get_tail());
                }
            }
            "U" => {
                for _ in 0..amount {
                    rope.move_up();
                    unique_locations.insert(rope.get_tail());
                }
            }
            "D" => {
                for _ in 0..amount {
                    rope.move_down();
                    unique_locations.insert(rope.get_tail());
                }
            }
            _ => {}
        }
    }
    // println!("Positions: {:?}", unique_locations);
    unique_locations.len().to_string()
}

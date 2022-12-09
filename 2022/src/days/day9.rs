use std::collections::HashSet;

struct Rope {
    head: (isize, isize), // x, bottom left = 0
    tail: (isize, isize), // y, bottom left = 0
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: (0, 0),
            tail: (0, 0),
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
        let hx: isize = self.head.0;
        let hy: isize = self.head.1;
        let tx: isize = self.tail.0;
        let ty: isize = self.tail.1;
        if (hx - tx).abs() > 1 || (hy - ty).abs() > 1 {
            // we need to update the tail
            if hx == tx {
                // only need to move y
                if hy < ty {
                    self.tail.1 -= 1;
                } else {
                    self.tail.1 += 1;
                }
            } else if hy == ty {
                // only need to move x
                if hx < tx {
                    self.tail.0 -= 1;
                } else {
                    self.tail.0 += 1;
                }
            } else {
                // need to move diagonally
                if hx > tx{
                    self.tail.0 += 1
                }else{
                    self.tail.0 -= 1;
                }

                if hy > ty{
                    self.tail.1 += 1
                }else{
                    self.tail.1 -= 1;
                }
            }
        }
    }

    fn get_tail(&self) -> (isize, isize) {
        self.tail
    }
}

#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> String {
    let mut rope = Rope::new();
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
    for line in input {}

    String::from("Placeholder part 2")
}
